use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent, gtk,
};

use crate::{
    page::Uri,
    view::{Screen, View, ViewMsg},
};

pub struct App {
    current_screen: Screen,
    view: Controller<View>,
}

#[derive(Debug)]
pub enum AppMsg {
    NavigateTo(Screen),
    Load(Uri),
    Unload(Uri),
}

#[relm4::component(pub)]
impl SimpleComponent for App {
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

                gtk::Button::with_label("Search") {
                    connect_clicked => AppMsg::NavigateTo(Screen::View)
                },

                gtk::Button::with_label("Add WebView") {
                    connect_clicked => AppMsg::Load(Uri { id: 0, source: "https://www.google.com/".into() })
                },

                gtk::Stack {
                    add_titled[Some(&Screen::Start.to_string()), "Start"] = &gtk::Label {
                        set_label: "Start",
                    },
                    add_titled[Some(&Screen::View.to_string()), "View"] = model.view.widget(),

                    #[watch]
                    set_visible_child_name: &model.current_screen.to_string(),
                }
           }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let view = View::builder().launch(0).detach();
        let model = App {
            current_screen: Screen::Start,
            view,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::NavigateTo(screen) => self.current_screen = screen,
            AppMsg::Load(uri) => self
                .view
                .sender()
                .send(ViewMsg::Load(uri))
                .expect("Error loading page"),
            AppMsg::Unload(uri) => self
                .view
                .sender()
                .send(ViewMsg::Unload(uri))
                .expect("Error unloading page"),
        }
    }
}
