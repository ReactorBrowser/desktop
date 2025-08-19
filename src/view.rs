use gtk::prelude::{BoxExt, OrientableExt};
use relm4::{
    factory::widgets, gtk::{self, prelude::WidgetExt}, prelude::{DynamicIndex, FactoryVecDeque}, ComponentParts, ComponentSender, Controller, RelmWidgetExt, SimpleComponent
};
use webkit6::{glib::property::PropertyGet, prelude::*};

use crate::{
    page::{Page, PageMsg},
    tab::{Tab, TabOutput},
};

pub struct View {
    pages: FactoryVecDeque<Page>,
    tabs: FactoryVecDeque<Tab>,
}

#[derive(Debug)]
pub enum ViewMsg {
    Open(String),
    Close(DynamicIndex),
    Show(DynamicIndex),
    Load(DynamicIndex, String),
    Unload(DynamicIndex),
    Back
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
                TabOutput::Close(index)=>ViewMsg::Close(index),
                TabOutput::Load(index,uri)=>ViewMsg::Load(index,uri),
                TabOutput::Show(index)=>ViewMsg::Show(index),
                TabOutput::Unload(index)=>ViewMsg::Unload(index),
            });

        let model = View { pages, tabs };

        let page_stack = model.pages.widget();
        let tab_box = model.tabs.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            ViewMsg::Open(uri) => {
                let index = self.tabs.guard().push_back(uri.clone());
                println!("{:?}", self.pages.widget().visible_child_name());
                sender.input(ViewMsg::Load(index, uri))
            }
            ViewMsg::Close(index) => {
                self.tabs.guard().remove(index.current_index());
                sender.input(ViewMsg::Unload(index));
            }
            ViewMsg::Back => {
            }
            ViewMsg::Show(index) => {
                self.pages
                    .widget()
                    .set_visible_child_name(&index.current_index().to_string());
            }
            ViewMsg::Load(index, uri) => {
                self.pages.guard().push_back((index, uri));
            }
            ViewMsg::Unload(index) => {
                let mut guard = self.pages.guard();
                let found = guard
                    .iter()
                    .position(|p| p.tab.current_index() == index.current_index());
                match found {
                    Some(index) => {
                        guard.remove(index);
                    }
                    None => eprintln!("Error unloading page"),
                }
            }
        }
    }
}
