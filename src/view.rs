pub mod panel;
pub mod tab;

use gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use panel::{Panel, PanelInput, PanelOutput};
use reactor_browser::{create_tab, delete_tab, establish_connection, show_tabs};
use relm4::{
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent, gtk, prelude::FactoryVecDeque,
};
use tab::{Tab, TabInput, TabOutput};
use webkit6::prelude::*;

use crate::icon_names;

pub struct View {
    tabs: FactoryVecDeque<Tab>,
    panels: FactoryVecDeque<Panel>,
    selected_tab_index: Option<usize>,
}

#[derive(Debug)]
pub enum ViewMsg {
    Open(String),
    Show(i32),
    Close(i32, bool),
    Load(i32, String),
    Unload(i32),
    Reload,
    GoBack,
    GoForward,
    UpdateTitle(i32, String),
}

#[relm4::component(pub)]
impl SimpleComponent for View {
    type Init = ();

    type Input = ViewMsg;
    type Output = ();

    view! {
        gtk::Paned {
            set_orientation: gtk::Orientation::Horizontal,
            set_position: 200,

            #[wrap(Some)]
            set_start_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 6,

                gtk::CenterBox {
                    #[wrap(Some)]
                    set_end_widget = &gtk::Box {
                        gtk::Button {
                            set_icon_name: icon_names::ARROW_CLOCKWISE_REGULAR,
                            connect_clicked => ViewMsg::Reload,
                        },
                        gtk::Button {
                            set_icon_name: icon_names::ARROW_REPLY_REGULAR,
                            connect_clicked => ViewMsg::GoBack,
                        },
                        gtk::Button {
                            set_icon_name: icon_names::ARROW_FORWARD_REGULAR,
                            connect_clicked => ViewMsg::GoForward,
                        }
                    }
                },

                gtk::Entry {
                    set_placeholder_text: Some("Search"),
                },
                gtk::Button::with_label("Open a new page") {
                    connect_clicked => ViewMsg::Open("https://www.google.com/".into()),
                },

                gtk::ScrolledWindow {
                    set_valign: gtk::Align::Fill,
                    set_vexpand: true,

                    #[local_ref]
                    tab_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 5,
                    }
                }
            },
            #[wrap(Some)]
            set_end_child = &gtk::Box {
                set_vexpand: true,
                set_margin_all: 16,
                set_margin_start: 0,

                #[local_ref]
                panel_stack -> gtk::Stack {}
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let tabs = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                TabOutput::Show(id) => ViewMsg::Show(id),
                TabOutput::Close(id, loaded) => ViewMsg::Close(id, loaded),
                TabOutput::Load(id, uri) => ViewMsg::Load(id, uri),
                TabOutput::Unload(id) => ViewMsg::Unload(id),
            });
        let panels = FactoryVecDeque::builder()
            .launch(gtk::Stack::default())
            .forward(sender.input_sender(), |output| match output {
                PanelOutput::UpdateTitle(id, title) => ViewMsg::UpdateTitle(id, title),
            });

        let connection = &mut establish_connection();

        let stored_tabs = show_tabs(connection).expect("Error loading tabs");
        let first_loaded_tab = stored_tabs.iter().position(|t| t.loaded);

        let mut model = View {
            tabs,
            panels,
            selected_tab_index: first_loaded_tab,
        };

        let panel_stack = model.panels.widget();
        let tab_box = model.tabs.widget();
        let widgets = view_output!();

        stored_tabs.iter().for_each(|s| {
            model.tabs.guard().push_back((s.id, s.url.clone()));
            if s.loaded {
                sender.input(ViewMsg::Load(s.id, s.url.clone()));
            }
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ViewMsg::Open(uri) => {
                let connection = &mut establish_connection();

                let tab =
                    create_tab(connection, &uri, "New tab".into()).expect("Error saving new tab");
                self.tabs.guard().push_back((tab.id, uri.clone()));

                sender.input(ViewMsg::Load(tab.id, uri));
            }
            ViewMsg::Show(id) => {
                self.panels.widget().set_visible_child_name(&id.to_string());
                self.selected_tab_index = self.panels.iter().position(|p| p.id == id);
            }
            ViewMsg::Close(id, loaded) => {
                let connection = &mut establish_connection();

                delete_tab(connection, id).expect("Error deleting tab");
                let mut guard = self.tabs.guard();
                let found = guard.iter().position(|t| t.id == id);
                match found {
                    Some(index) => {
                        guard.remove(index);
                    }
                    None => eprintln!("Error closing tab"),
                }

                if loaded {
                    sender.input(ViewMsg::Unload(id));
                }
            }
            ViewMsg::Load(id, uri) => {
                self.panels.guard().push_back((id, uri));
            }
            ViewMsg::Unload(id) => {
                let mut guard = self.panels.guard();
                let found = guard.iter().position(|p| p.id == id);
                match found {
                    Some(index) => {
                        guard.remove(index);
                    }
                    None => eprintln!("Error unloading page"),
                }
            }
            ViewMsg::Reload => {
                if let Some(index) = self.selected_tab_index {
                    self.panels.send(index, PanelInput::Reload);
                }
            }
            ViewMsg::GoBack => {
                if let Some(index) = self.selected_tab_index {
                    self.panels.send(index, PanelInput::GoBack)
                }
            }
            ViewMsg::GoForward => {
                if let Some(index) = self.selected_tab_index {
                    self.panels.send(index, PanelInput::GoForward);
                }
            }
            ViewMsg::UpdateTitle(id, title) => {
                let found = self.tabs.iter().position(|t| t.id == id);
                match found {
                    Some(index) => {
                        self.tabs.send(index, TabInput::UpdateTitle(title));
                    }
                    None => eprintln!("Error changing tab title"),
                }
            }
        }
    }
}
