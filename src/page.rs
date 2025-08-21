use gtk::prelude::WidgetExt;
use reactor_browser::{create_history, establish_connection};
use relm4::{FactorySender, factory, gtk, prelude::FactoryComponent};
use webkit6::{LoadEvent, Settings, WebView, prelude::WebViewExt};

pub struct Page {
    pub tab_id: i32,
    pub uri: String,
}

#[derive(Debug)]
pub enum PageMsg {
    GoBack,
    GoForward,
    Reload,
    UpdateNavState,
}

#[relm4::factory(pub)]
impl FactoryComponent for Page {
    type Init = (i32, String);

    type Input = PageMsg;
    type Output = ();

    type CommandOutput = ();
    type ParentWidget = gtk::Stack;

    view! {
        #[name(webview)]
        WebView {
            load_uri: &self.uri,
            add_css_class: "page-box-wrapper",
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

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        let (tab_id, uri) = init;

        Self { tab_id, uri }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        returned_widget: &<Self::ParentWidget as factory::FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let returned_widget = returned_widget.clone();
        returned_widget.set_name(&self.tab_id.to_string());
        // root.connect_title_notify(move |webview| {});

        let widgets = view_output!();
        widgets
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        _sender: FactorySender<Self>,
    ) {
        let webview = &widgets.webview;
        match message {
            PageMsg::GoBack => {
                webview.go_back();
            }
            PageMsg::GoForward => {
                webview.go_forward();
            }
            PageMsg::Reload => {
                webview.reload();
            }
            PageMsg::UpdateNavState => {}
        }
    }
}
