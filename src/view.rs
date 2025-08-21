use gtk::prelude::{BoxExt, OrientableExt, WidgetExt};
use reactor_browser::{create_tab, delete_tab, establish_connection, show_tabs};
use relm4::{
    ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent, gtk, prelude::FactoryVecDeque,
};
use webkit6::prelude::*;

use crate::{
    page::Page,
    tab::{Tab, TabOutput},
};

pub struct View {
    pages: FactoryVecDeque<Page>,
    tabs: FactoryVecDeque<Tab>,
}

#[derive(Debug)]
pub enum ViewMsg {
    Open(String),
    Show(i32),
    Close(i32, bool),
    Load(i32, String),
    Unload(i32),
    Back,
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
                    set_margin_all: 10,
                    set_spacing: 5,
                    gtk::CenterBox {
                        #[wrap(Some)]
                        set_start_widget = &gtk::Box {
                            gtk::Button {}
                        },
                        #[wrap(Some)]
                        set_end_widget = &gtk::Box {
                            add_css_class: "linked",
                            #[name = "back"]
                            gtk::Button {
                                set_icon_name?: Some("arrow-left-symbolic"),

                            },
                            #[name = "forward"]
                            gtk::Button {
                                set_icon_name?: Some("arrow-right-symbolic"),

                            },
                            #[name = "reload"]
                            gtk::Button {
                                set_icon_name?: Some("collection-rescan-amarok-symbolic"),
                            },
                        },
                    },

                    gtk::Entry {
                        set_placeholder_text: Some("Search"),
                    },

                    gtk::Button::with_label("Open new page") {
                        connect_clicked => ViewMsg::Open("https://www.google.com/".into())
                    },

                    gtk::ScrolledWindow {
                        set_valign: gtk::Align::Fill,
                        set_vexpand: true,
                        #[local_ref]
                        tab_box -> gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 5,
                        },
                    },

            },
            #[wrap(Some)]
            set_end_child = &gtk::Box {
                set_vexpand: true,
                add_css_class: "page-box-wrapper",
                set_margin_all: 16,
                set_margin_start: 0,


                #[local_ref]
                page_stack -> gtk::Stack {
                    add_css_class: "page-box",
                },
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let pages = FactoryVecDeque::builder()
            .launch(gtk::Stack::default())
            .detach();
        let tabs = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .forward(sender.input_sender(), |output| match output {
                TabOutput::Show(id) => ViewMsg::Show(id),
                TabOutput::Close(id, loaded) => ViewMsg::Close(id, loaded),
                TabOutput::Load(id, uri) => ViewMsg::Load(id, uri),
                TabOutput::Unload(id) => ViewMsg::Unload(id),
            });

        let mut model = View { pages, tabs };

        let page_stack = model.pages.widget();
        let tab_box = model.tabs.widget();
        let widgets = view_output!();

        let connection = &mut establish_connection();

        let stored_tabs = show_tabs(connection).expect("Error loading tabs");
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

                sender.input(ViewMsg::Load(tab.id, uri))
            }
            ViewMsg::Back => {}
            ViewMsg::Show(id) => self.pages.widget().set_visible_child_name(&id.to_string()),
            ViewMsg::Close(id, loaded) => {
                let connection = &mut establish_connection();

                delete_tab(connection, id).expect("Error deleting tab");
                let mut guard = self.tabs.guard();
                let found = guard.iter().position(|t| t.id == id);
                match found {
                    Some(id) => {
                        guard.remove(id);
                    }
                    None => eprintln!("Error closing tab"),
                }

                if loaded {
                    sender.input(ViewMsg::Unload(id));
                }
            }
            ViewMsg::Load(tab_id, uri) => {
                self.pages.guard().push_back((tab_id, uri));
            }
            ViewMsg::Unload(id) => {
                let mut guard = self.pages.guard();
                let found = guard.iter().position(|p| p.tab_id == id);
                match found {
                    Some(id) => {
                        guard.remove(id);
                    }
                    None => eprintln!("Error unloading page"),
                }
            }
        }
    }
}
