use gtk::prelude::{BoxExt, OrientableExt};
use relm4::{
    ComponentParts, ComponentSender, SimpleComponent,
    gtk::{self, prelude::WidgetExt},
    prelude::FactoryVecDeque,
};

use crate::page::{Page, Uri};

pub struct View {
    created_widgets: u32,
    pages: FactoryVecDeque<Page>,
}

#[derive(Debug)]
pub enum ViewMsg {
    Load(Uri),
    Unload(Uri),
}

#[relm4::component(pub)]
impl SimpleComponent for View {
    type Init = u32;

    type Input = ViewMsg;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            gtk::StackSwitcher {
                set_stack: Some(&page_box)
            },

            gtk::Box {
                set_vexpand: true,
                add_css_class: "page-box-wrapper",

                #[local_ref]
                page_box -> gtk::Stack {
                    add_css_class: "page-box",
                },
            }
        }
    }

    fn init(
        page: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let pages = FactoryVecDeque::builder()
            .launch(gtk::Stack::default())
            .detach();

        let model = View {
            created_widgets: page,
            pages,
        };

        let page_box = model.pages.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ViewMsg::Load(uri) => {
                self.pages.guard().push_back(uri);
                self.created_widgets = self.created_widgets.wrapping_add(1);
            }
            ViewMsg::Unload(uri) => {
                let mut guard = self.pages.guard();
                let found = guard.iter().position(|page| page.uri.id == uri.id);
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
