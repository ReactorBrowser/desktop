mod app;
mod page;
mod tab;
mod view;

use relm4::RelmApp;

use app::App;

mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn main() {
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);

    let app = RelmApp::new("xyz.reactor.browser");
    relm4::set_global_css_from_file("data/resources/style.css").unwrap();
    app.run::<App>(());
}
