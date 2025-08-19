use gtk::prelude::ButtonExt;
use reactor_browser::{establish_connection, load_tab, unload_tab};
use relm4::{gtk, prelude::FactoryComponent};

pub struct Tab {
    pub id: i32,
    uri: String,
    loaded: bool,
}

#[derive(Debug)]
pub enum TabInput {
    Close,
    Load,
    Unload,
}

#[derive(Debug)]
pub enum TabOutput {
    Close(i32, bool),
    Load(i32, String),
    Unload(i32),
}

#[relm4::factory(pub)]
impl FactoryComponent for Tab {
    type Init = (i32, String);

    type Input = TabInput;
    type Output = TabOutput;

    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box {
            gtk::Button::with_label("Close") {
                connect_clicked => TabInput::Close
            },

            if self.loaded {
                gtk::Button::with_label("Unload") {
                    connect_clicked => TabInput::Unload
                }
            } else {
                gtk::Button::with_label("Load") {
                    connect_clicked => TabInput::Load
                }
            },
        }
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        let (id, uri) = init;
        Self {
            id,
            uri,
            loaded: true,
        }
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::FactorySender<Self>) {
        match msg {
            TabInput::Close => sender
                .output(TabOutput::Close(self.id, self.loaded))
                .unwrap(),
            TabInput::Load => {
                let connection = &mut establish_connection();

                load_tab(connection, self.id).expect("Error loading tab");

                self.loaded = true;
                sender
                    .output(TabOutput::Load(self.id, self.uri.clone()))
                    .unwrap();
            }
            TabInput::Unload => {
                let connection = &mut establish_connection();

                unload_tab(connection, self.id).expect("Error unloading tab");

                self.loaded = false;
                sender.output(TabOutput::Unload(self.id)).unwrap();
            }
        }
    }
}
