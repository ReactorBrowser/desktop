mod app;
mod view;

use app::App;
use gtk::{gdk, gio};
use reactor_browser::{establish_connection, run_migrations};
use relm4::{RelmApp, gtk};

mod icon_names {
    pub use shipped::*;
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn initialize_custom_styles() {
    gio::resources_register_include!("styles.gresource").unwrap();

    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/xyz/reactor/browser/style.css");

    let display = gdk::Display::default().unwrap();
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = RelmApp::new("xyz.reactor.browser");

    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
    initialize_custom_styles();

    {
        let connection = &mut establish_connection();
        run_migrations(connection).expect("Error running database migrations");
    }

    app.run::<App>(());
}
