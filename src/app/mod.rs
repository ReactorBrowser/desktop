pub mod config;
pub mod message;
pub mod ui;

use iced::{Subscription, Task, window};
use iced_wry::WebViewController;
use reactor_browser::{
    create_history, create_tab, delete_tab, establish_connection, models::Tab, run_migrations,
    show_tabs, update_tab,
};
use std::collections::HashMap;

use config::create_webview_config;
use message::Message;

pub struct App {
    pub url_input: String,
    pub tabs: Vec<Tab>,
    pub active_tab_id: Option<i32>,
    pub controllers: HashMap<i32, WebViewController>,
    pub window_id: Option<window::Id>,
    pub tab_progress: HashMap<i32, f32>,
}

impl App {
    pub fn title(&self) -> String {
        if let Some(id) = self.active_tab_id {
            if let Some(tab) = self.tabs.iter().find(|t| t.id == id) {
                if let Some(title) = &tab.title {
                    if !title.is_empty() {
                        return format!("{title} - Reactor Browser");
                    }
                }
            }
        }
        String::from("Reactor Browser")
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn new() -> (Self, Task<Message>) {
        let connection = &mut establish_connection().expect("Failed to connect to database");
        run_migrations(connection).expect("Error running database migrations");

        let mut tabs = show_tabs(connection).unwrap_or_default();
        if tabs.is_empty() {
            if let Ok(new_tab) = create_tab(connection, "https://www.google.com", Some("New Tab")) {
                tabs.push(new_tab);
            }
        }

        let active_tab_id = tabs.first().map(|t| t.id);

        let url_input = if let Some(id) = active_tab_id {
            tabs.iter()
                .find(|t| t.id == id)
                .map(|t| t.url.clone())
                .unwrap_or_default()
        } else {
            String::from("https://www.google.com")
        };

        let mut controllers = HashMap::new();
        for tab in &tabs {
            controllers.insert(
                tab.id,
                WebViewController::new(create_webview_config(&tab.url)),
            );
        }

        (
            Self {
                url_input,
                tabs,
                active_tab_id,
                controllers,
                window_id: None,
                tab_progress: HashMap::new(),
            },
            window::oldest().map(Message::GotWindow),
        )
    }

    pub fn update_visibilities(&self) {
        for (id, controller) in &self.controllers {
            controller.set_visible(Some(*id) == self.active_tab_id);
        }
    }

    pub fn run_js(&self, script: &str) {
        if let Some(id) = self.active_tab_id {
            if let Some(controller) = self.controllers.get(&id) {
                controller.evaluate_script(script);
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let ipc_subs = self.controllers.iter().map(|(id, controller)| {
            let tab_id = *id;
            controller
                .ipc_subscription()
                .with(tab_id)
                .map(|(id, msg)| Message::Ipc(id, msg.body))
        });

        let mut subs = vec![Subscription::batch(ipc_subs)];

        if !self.tab_progress.is_empty() {
            subs.push(
                iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick),
            );
        }

        Subscription::batch(subs)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Ipc(tab_id, body) => {
                #[derive(serde::Deserialize)]
                struct IpcPayload {
                    #[serde(rename = "type")]
                    msg_type: String,
                    payload: String,
                }

                if let Ok(data) = serde_json::from_str::<IpcPayload>(&body) {
                    let connection =
                        &mut establish_connection().expect("Failed to connect to database");
                    let mut update_needed = false;

                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                        if data.msg_type == "title"
                            && tab.title.as_deref() != Some(&data.payload)
                            && !data.payload.is_empty()
                        {
                            tab.title = Some(data.payload.clone());
                            update_needed = true;
                        } else if data.msg_type == "url" && tab.url != data.payload {
                            tab.url = data.payload.clone();
                            if self.active_tab_id == Some(tab_id) {
                                self.url_input = tab.url.clone();
                            }
                            update_needed = true;
                            let _ = create_history(connection, &data.payload, tab.title.as_deref());
                        } else if data.msg_type == "load_start" {
                            self.tab_progress.insert(tab_id, 10.0);
                        } else if data.msg_type == "load_finish" {
                            self.tab_progress.insert(tab_id, 100.0);
                        }

                        if update_needed {
                            let _ = update_tab(connection, tab_id, &tab.url, tab.title.as_deref());
                        }
                    }
                }
                Task::none()
            }
            Message::Tick => {
                for progress in self.tab_progress.values_mut() {
                    if *progress >= 100.0 {
                        *progress += 2.0;
                    } else {
                        *progress += (95.0 - *progress) * 0.1;
                    }
                }
                self.tab_progress.retain(|_, p| *p < 105.0);
                Task::none()
            }
            Message::GotWindow(Some(id)) => {
                self.window_id = Some(id);
                let mut tasks = Vec::new();
                for (&tab_id, controller) in self.controllers.iter_mut() {
                    tasks.push(
                        controller
                            .create_task(id, Message::WebViewReadyGeneric)
                            .map(move |msg| match msg {
                                Message::WebViewReadyGeneric(res) => {
                                    Message::WebViewReady(tab_id, res)
                                }
                                _ => unreachable!(),
                            }),
                    );
                }
                Task::batch(tasks)
            }
            Message::GotWindow(None) => Task::none(),
            Message::WebViewReady(id, Ok(())) => {
                if let Some(controller) = self.controllers.get_mut(&id) {
                    controller.take_staged();
                    controller.set_visible(Some(id) == self.active_tab_id);
                }
                Task::none()
            }
            Message::WebViewReady(id, Err(e)) => {
                eprintln!("WebView {} error: {}", id, e);
                Task::none()
            }
            Message::UrlChanged(url) => {
                self.url_input = url;
                Task::none()
            }
            Message::GoPressed => {
                if let Some(id) = self.active_tab_id {
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == id) {
                        tab.url = self.url_input.clone();
                    }
                    self.tab_progress.insert(id, 10.0);
                }
                self.run_js(&format!("window.location.href = '{}'", self.url_input));
                Task::none()
            }
            Message::BackPressed => {
                if let Some(id) = self.active_tab_id {
                    self.tab_progress.insert(id, 10.0);
                }
                self.run_js("window.history.back()");
                Task::none()
            }
            Message::ForwardPressed => {
                if let Some(id) = self.active_tab_id {
                    self.tab_progress.insert(id, 10.0);
                }
                self.run_js("window.history.forward()");
                Task::none()
            }
            Message::ReloadPressed => {
                if let Some(id) = self.active_tab_id {
                    self.tab_progress.insert(id, 10.0);
                }
                self.run_js("location.reload()");
                Task::none()
            }
            Message::TabSelected(id) => {
                self.active_tab_id = Some(id);
                if let Some(tab) = self.tabs.iter().find(|t| t.id == id) {
                    self.url_input = tab.url.clone();
                }
                self.update_visibilities();
                Task::none()
            }
            Message::NewTab => {
                let connection =
                    &mut establish_connection().expect("Failed to connect to database");
                if let Ok(new_tab) =
                    create_tab(connection, "https://www.google.com", Some("New Tab"))
                {
                    let new_id = new_tab.id;
                    self.active_tab_id = Some(new_id);
                    self.url_input = new_tab.url.clone();
                    self.tabs.push(new_tab);
                    self.tab_progress.insert(new_id, 10.0);

                    let mut new_controller =
                        WebViewController::new(create_webview_config(&self.url_input));
                    let mut attach_task = Task::none();
                    if let Some(window_id) = self.window_id {
                        attach_task = new_controller
                            .create_task(window_id, Message::WebViewReadyGeneric)
                            .map(move |msg| match msg {
                                Message::WebViewReadyGeneric(res) => {
                                    Message::WebViewReady(new_id, res)
                                }
                                _ => unreachable!(),
                            });
                    }

                    self.controllers.insert(new_id, new_controller);
                    self.update_visibilities();

                    attach_task
                } else {
                    Task::none()
                }
            }
            Message::CloseTab(id) => {
                let connection =
                    &mut establish_connection().expect("Failed to connect to database");
                let _ = delete_tab(connection, id);
                self.tabs.retain(|t| t.id != id);
                self.controllers.remove(&id);

                if self.active_tab_id == Some(id) {
                    if let Some(first_tab) = self.tabs.first() {
                        self.active_tab_id = Some(first_tab.id);
                        self.url_input = first_tab.url.clone();
                    } else {
                        self.active_tab_id = None;
                        self.url_input.clear();
                    }
                }
                self.update_visibilities();
                Task::none()
            }
            Message::WebViewReadyGeneric(_) => unreachable!(),
        }
    }
}
