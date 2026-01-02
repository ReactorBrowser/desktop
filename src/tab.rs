use gtk::prelude::{ButtonExt, WidgetExt};
use reactor_browser::{establish_connection, load_tab, unload_tab};
use relm4::{gtk, prelude::FactoryComponent};

use crate::icon_names;

pub struct Tab {
    pub id: i32,
    title: String,
    uri: String,
    loaded: bool,
}

#[derive(Debug)]
pub enum TabInput {
    Show,
    Close,
    Load,
    Unload,
    UpdateTitle(String),
}

#[derive(Debug)]
pub enum TabOutput {
    Show(i32),
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
            gtk::Button {
                add_css_class: "tab",
                set_halign: gtk::Align::Fill,
                set_hexpand: true,
                #[watch]
                set_label: &self.title,
                connect_clicked => TabInput::Show,
            },

            gtk::Button {
                set_icon_name: icon_names::DISMISS_REGULAR,
                add_css_class: "icon",
                connect_clicked => TabInput::Close,
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
            title: "...".to_string(),
            uri,
            loaded: true,
        }
    }

    fn update(&mut self, msg: Self::Input, sender: relm4::FactorySender<Self>) {
        match msg {
            TabInput::Show => sender.output(TabOutput::Show(self.id)).unwrap(),
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
            TabInput::UpdateTitle(title) => {
                if title.len() > 20 {
                    self.title = String::from(&title[..20]) + "...";
                } else {
                    self.title = title;
                }
            }
        }
    }
}
