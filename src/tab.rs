use gtk::prelude::ButtonExt;
use relm4::{
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

pub struct Tab {
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
            gtk::Button::with_label("Close") {
                connect_clicked[sender, index] => move |_| {
                    sender.output(TabOutput::Close(index.clone())).unwrap();
                }
            },

            if self.loaded {
                gtk::Button::with_label("Unload") {
                    connect_clicked[sender, index] => move |_| {
                        sender.input(TabInput::Unload(index.clone()));
                    }
                }
            } else {
                gtk::Button::with_label("Load") {
                    connect_clicked[sender, index] => move |_| {
                        sender.input(TabInput::Load(index.clone()));
                    }
                }
            },
        }
    }

    fn init_model(
        uri: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self { uri, loaded: true }
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
