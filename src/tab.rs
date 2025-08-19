use gtk::prelude::ButtonExt;
use relm4::{
    gtk::{self, prelude::WidgetExt},
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::view::{View, ViewMsg};

pub struct Tab {
    title: String,
    uri: String,
    loaded: bool,
}

#[derive(Debug)]
pub enum TabInput {
    Load(DynamicIndex),
    Unload(DynamicIndex),
}

#[derive(Debug)]
pub enum TabOutput {
    Close(DynamicIndex),
    Load(DynamicIndex, String),
    Show(DynamicIndex),
    Unload(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for Tab {
    type Init = String;

    type Input = TabInput;
    type Output = TabOutput;

    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            
            add_css_class: "linked",
            gtk::Button {
                add_css_class: "tab",
                set_halign: gtk::Align::Fill,
                set_hexpand: true,
                #[watch]
                set_label: "Title",
                connect_clicked[sender, index] => move |_| {
                    sender.output(TabOutput::Show(index.clone())).unwrap();
                }
            },
            gtk::Button::with_label("Close") {
                connect_clicked[sender, index] => move |_| {
                    sender.output(TabOutput::Close(index.clone())).unwrap();
                }
            },
        }

    }

    fn init_model(
        uri: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self { uri, title: " ".to_string(), loaded: true }
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::FactorySender<Self>) {
        match msg {
            TabInput::Load(index) => {
                self.loaded = true;
                sender
                    .output(TabOutput::Load(index, self.uri.clone()))
                    .unwrap();
            }
            TabInput::Unload(index) => {
                self.loaded = false;
                sender.output(TabOutput::Unload(index)).unwrap();
            }
        }
    }
}
