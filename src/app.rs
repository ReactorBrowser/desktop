use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent, gtk,
};

use crate::view::{View, ViewMsg};

pub struct App {
    view: Controller<View>,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenTab,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Reactor"),
            add_css_class: "root",

           gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 16,

                gtk::Button::with_label("Open new page") {
                    connect_clicked => AppMsg::OpenTab
                },

                append = model.view.widget(),
           }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let view = View::builder().launch(()).detach();

        let model = App { view };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::OpenTab => self
                .view
                .emit(ViewMsg::Open("https://www.google.com/".into())),
        }
    }
}
