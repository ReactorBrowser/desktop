use gtk::prelude::{BoxExt, GtkWindowExt, OrientableExt, WidgetExt};
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, SimpleComponent,
    gtk,
};

use crate::view::View;

pub struct App {
    view: Controller<View>,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();

    type Input = ();
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Reactor"),
            add_css_class: "root",
            set_default_width: 1000,
            set_default_height: 800,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                append = model.view.widget(),
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let view = View::builder().launch(()).detach();

        let model = App { view };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
