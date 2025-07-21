use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use relm4::{
    ComponentParts, RelmApp, RelmWidgetExt, SimpleComponent,
    gtk::{self, prelude::GtkWindowExt},
};
use std::fmt;

#[derive(Debug)]
enum Screen {
    Start,
    Settings,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => "start",
                Self::Settings => "settings",
            }
        )
    }
}

struct AppModel {
    current_screen: Screen,
}

#[derive(Debug)]
enum AppMsg {
    NavigateTo(Screen),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Reactor"),

           gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 16,
                set_margin_all: 16,

                gtk::Button::with_label("Go to settings") {
                    connect_clicked => AppMsg::NavigateTo(Screen::Settings)
                },

                gtk::Stack {
                    add_titled: (&gtk::Label::new(Some("Start")), Some(&Screen::Start.to_string()), "Start"),
                    add_titled: (&gtk::Label::new(Some("Settings")), Some(&Screen::Settings.to_string()), "Settings"),

                    #[watch]
                    set_visible_child_name: &model.current_screen.to_string()
                }
           }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel {
            current_screen: Screen::Start,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match msg {
            AppMsg::NavigateTo(screen) => self.current_screen = screen,
        }
    }
}

fn main() {
    let app = RelmApp::new("xyz.reactor.browser");
    app.run::<AppModel>(());
}
