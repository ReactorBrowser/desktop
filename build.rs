use glib_build_tools::compile_resources;

fn main() {
    compile_resources(&["data"], "data/styles.gresource.xml", "styles.gresource");

    relm4_icons_build::bundle_icons(
        // Name of the file that will be generated at `OUT_DIR`
        "icon_names.rs",
        // Optional app ID
        Some("xyz.reactor.browser"),
        // Custom base resource path:
        // * defaults to `/com/example/myapp` in this case if not specified explicitly
        // * or `/org/relm4` if app ID was not specified either
        None::<&str>,
        // Directory with custom icons (if any)
        None::<&str>,
        // List of icons to include
        [
            "arrow-clockwise-regular",
            "arrow-reply-regular",
            "arrow-forward-regular",
            "dismiss-regular",
        ],
    );
}
