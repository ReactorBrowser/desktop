mod app;
mod page;
mod tab;
mod view;

use relm4::RelmApp;

use app::App;

fn main() {
    let app = RelmApp::new("xyz.reactor.browser");
    relm4::set_global_css_from_file("data/resources/style.css").unwrap();
    app.run::<App>(());
}
