use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent, gtk,
};

use crate::{
    page::Uri,
    view::{View, ViewMsg},
};

pub struct App {
    view: Controller<View>,
}

#[derive(Debug)]
pub enum AppMsg {
    Open(Uri),
    Unload(Uri),
    Close(Uri),
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
                set_margin_all: 16,

                gtk::Button::with_label("Open new page") {
                    connect_clicked => AppMsg::Open(Uri { id: 0, source: "https://www.google.com/".into() })
                },

                append = model.view.widget(),
           }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let view = View::builder().launch(0).detach();
        let model = App { view };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Open(uri) => self
                .view
                .sender()
                .send(ViewMsg::Load(uri))
                .expect("Error loading page"),
            AppMsg::Unload(uri) => self
                .view
                .sender()
                .send(ViewMsg::Unload(uri))
                .expect("Error closing page"),
            AppMsg::Close(uri) => self
                .view
                .sender()
                .send(ViewMsg::Unload(uri))
                .expect("Error unloading page"),
        }
    }
}
