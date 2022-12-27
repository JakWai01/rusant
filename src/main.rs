mod call_window;
mod ports;
mod receiver;
mod sender;

use call_window::CallWindow;

use config::Config;
use glib::clone;
use gtk::{gdk::Display, glib, CssProvider, StyleContext};
use std::collections::HashMap;
use std::path::Path;
use gtk::prelude::ActionMapExt;
use gtk_macros::action;
use gtk::prelude::GtkApplicationExt;
use gtk::prelude::GtkWindowExt;

use gtk::gio::resources_register_include;

use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, HeaderBar, WindowTitle
};

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Parse config
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let name = &config.get("name").unwrap();
    let app_id = &config.get("app_id").unwrap();

    // Check if video device exists
    if !Path::new("/dev/video0").exists() {
        panic!("No webcam detected: /dev/video0 cannot be found.")
    }

    // Initialize GTK
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    // Initialize GStreamer
    gst::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Initialize variables
    glib::set_application_name(name);
    gtk::Window::set_default_icon_name(app_id);

    gtk::init().unwrap();

    // Load gst-plugin-gtk4 GStreamer plugin
    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Load resources
    resources_register_include!("gtk-rusant.gresource").expect("Failed to register resources.");

    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_resource("./style.css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Initialize application
    let app = Application::builder().application_id(app_id).build();

    // Run application
    app.connect_activate(build_ui);
    let actions = gio::SimpleActionGroup::new();
    app.set_action_group(Some(&actions));
    action!{
        actions,
        "about",
        clone!(@weak app as app => move |_, _| {
           show_about(&app);
        })
    };
    
    app.run();

    // Deinitialize GStreamer
    unsafe {
        gst::deinit();
    }
}

fn build_ui(app: &Application) {
    let content = libadwaita::gtk::Box::new(Orientation::Vertical, 0);

    content.append(
        &HeaderBar::builder()
            .title_widget(&WindowTitle::new("Rusant", ""))
            .build(),
    );

    let window = CallWindow::new(app);

    window.show();
}

fn show_about(app: &Application) {
    let window = app.active_window().unwrap();
    let dialog = libadwaita::AboutWindow::builder()
        .transient_for(&window)
        // .application_icon("rusant")
        .application_name("Rusant")
        .developer_name("Jakob Waibel")
        .version("0.0.1")
        .developers(vec!["Jakob Waibel".into(), "Felicitas Pojtinger".into()])
        .copyright("© 2022 Jakob Waibel")
        .website("https://github.com/JakWai01/rusant")
        .issue_url("https://github.com/JakWai01/rusant/issues/new")
        .license_type(gtk::License::Agpl30)
        .build();

    dialog.present();
}