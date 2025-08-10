use std::fmt;

use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use reactor_browser::*;
use relm4::{
    Component, ComponentController, ComponentParts, Controller, RelmApp, RelmWidgetExt,
    SimpleComponent, gtk,
    prelude::{FactoryComponent, FactoryVecDeque},
};
use webkit6::{LoadEvent, Settings, WebView, prelude::WebViewExt};

#[derive(Debug)]
struct Uri {
    id: u32,
    source: String,
}

struct PageModel {
    uri: Uri,
}

#[relm4::factory]
impl FactoryComponent for PageModel {
    type Init = Uri;

    type Input = ();
    type Output = ();

    type CommandOutput = ();
    type ParentWidget = gtk::Stack;

    view! {
        WebView {
            load_uri: &self.uri.source,

            set_valign: gtk::Align::Fill,
            set_vexpand: true,
            set_halign: gtk::Align::Fill,
            set_hexpand: true,

            set_settings = &Settings {
                set_enable_developer_extras: true,
                set_enable_write_console_messages_to_stdout: true
            },

            connect_load_changed => |webview, event| {
                if let LoadEvent::Finished = event {
                    let connection = &mut establish_connection();
                    create_history(
                        connection,
                        webview.uri().expect("Error getting webview URI").as_str(),
                        webview.title().as_ref().map(|t| t.as_str()),
                    ).expect("Error creating new history");
                }
            },
        }
    }

    fn init_model(
        uri: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self { uri }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: relm4::FactorySender<Self>,
    ) -> Self::Widgets {
        let returned_widget = returned_widget.clone();
        root.connect_title_notify(move |webview| {
            if let Some(title) = webview.title() {
                returned_widget.set_title(title.as_str());
            }
        });

        let widgets = view_output!();
        widgets
    }
}

#[derive(Clone, Copy, Debug)]
enum Screen {
    Start,
    View,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start => "start",
                Self::View => "view",
            }
        )
    }
}

struct ViewModel {
    created_widgets: u32,
    pages: FactoryVecDeque<PageModel>,
}

#[derive(Debug)]
enum ViewMsg {
    Load(Uri),
    Unload(Uri),
}

#[relm4::component]
impl SimpleComponent for ViewModel {
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

            #[local_ref]
            page_box -> gtk::Stack {},
        }
    }

    fn init(
        page: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let pages = FactoryVecDeque::builder()
            .launch(gtk::Stack::default())
            .detach();

        let model = ViewModel {
            created_widgets: page,
            pages,
        };

        let page_box = model.pages.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::ComponentSender<Self>) {
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

struct AppModel {
    current_screen: Screen,
    view: Controller<ViewModel>,
}

#[derive(Debug)]
enum AppMsg {
    NavigateTo(Screen),
    Load(Uri),
    Unload(Uri),
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
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let view = ViewModel::builder().launch(0).detach();
        let model = AppModel {
            current_screen: Screen::Start,
            view,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: relm4::ComponentSender<Self>) {
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

fn main() {
    let app = RelmApp::new("xyz.reactor.browser");
    app.run::<AppModel>(());
}
