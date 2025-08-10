mod app;
mod page;
mod view;

use relm4::RelmApp;

use app::App;

fn main() {
    let app = RelmApp::new("xyz.reactor.browser");
    app.run::<App>(());
}
