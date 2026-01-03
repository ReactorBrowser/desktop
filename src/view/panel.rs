use gtk::prelude::WidgetExt;
use reactor_browser::{create_history, establish_connection};
use relm4::{FactorySender, factory, gtk, prelude::FactoryComponent};
use webkit6::{LoadEvent, Settings, WebView, prelude::WebViewExt};

pub struct Panel {
    pub id: i32,
    uri: String,
}

#[derive(Debug)]
pub enum PanelInput {
    Reload,
    GoBack,
    GoForward,
    UpdateTitle,
}

#[derive(Debug)]
pub enum PanelOutput {
    UpdateTitle(i32, String),
}

#[relm4::factory(pub)]
impl FactoryComponent for Panel {
    type Init = (i32, String);

    type Input = PanelInput;
    type Output = PanelOutput;

    type CommandOutput = ();
    type ParentWidget = gtk::Stack;

    view! {
        #[name(webview)]
        WebView {
            load_uri: &self.uri,
            add_css_class: "page-box",
            set_valign: gtk::Align::Fill,
            set_vexpand: true,
            set_halign: gtk::Align::Fill,
            set_hexpand: true,
            set_settings = &Settings {
                set_enable_developer_extras: true,
                set_enable_write_console_messages_to_stdout: true,
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
            connect_title_notify => PanelInput::UpdateTitle,
        }
    }

    fn init_model(init: Self::Init, _index: &Self::Index, _sender: FactorySender<Self>) -> Self {
        let (id, uri) = init;

        Self { id, uri }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: Self::Root,
        returned_widget: &<Self::ParentWidget as factory::FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let returned_widget = returned_widget.clone();
        returned_widget.set_name(&self.id.to_string());

        let widgets = view_output!();
        widgets
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: FactorySender<Self>,
    ) {
        let webview = &widgets.webview;
        match message {
            PanelInput::GoBack => {
                webview.go_back();
            }
            PanelInput::GoForward => {
                webview.go_forward();
            }
            PanelInput::Reload => {
                webview.reload();
            }
            PanelInput::UpdateTitle => {
                sender
                    .output(PanelOutput::UpdateTitle(
                        self.id,
                        webview.title().unwrap_or_default().into(),
                    ))
                    .unwrap();
            }
        }
    }
}
