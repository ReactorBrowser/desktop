use gtk::prelude::{BoxExt, OrientableExt};
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    gtk::{self, prelude::WidgetExt},
    prelude::{DynamicIndex, FactoryVecDeque},
};

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
    Close(DynamicIndex),
    Load(DynamicIndex, String),
    Unload(DynamicIndex),
}

#[relm4::component(pub)]
impl SimpleComponent for View {
    type Init = ();

    type Input = ViewMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            #[local_ref]
            tab_box -> gtk::Box {
                set_spacing: 16,
            },

            gtk::StackSwitcher {
                set_stack: Some(&page_stack)
            },

            gtk::Box {
                set_vexpand: true,
                add_css_class: "page-box-wrapper",

                #[local_ref]
                page_stack -> gtk::Stack {
                    add_css_class: "page-box",
                },
            }
        }
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
                TabOutput::Close(index) => ViewMsg::Close(index),
                TabOutput::Load(index, uri) => ViewMsg::Load(index, uri),
                TabOutput::Unload(index) => ViewMsg::Unload(index),
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
                sender.input(ViewMsg::Load(index, uri))
            }
            ViewMsg::Close(index) => {
                self.tabs.guard().remove(index.current_index());
                sender.input(ViewMsg::Unload(index));
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
