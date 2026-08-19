use iced::window;

#[derive(Debug, Clone)]
pub enum Message {
    UrlChanged(String),
    GoPressed,
    BackPressed,
    ForwardPressed,
    ReloadPressed,
    TabSelected(i32),
    NewTab,
    CloseTab(i32),
    GotWindow(Option<window::Id>),
    WebViewReadyGeneric(Result<(), String>),
    WebViewReady(i32, Result<(), String>),
    Ipc(i32, String),
    Tick,
}
