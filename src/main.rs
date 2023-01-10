mod ports;
mod receiver;
mod rusant_call_pane;
mod rusant_contact_dialog;
mod rusant_contact_item;
mod rusant_contact_list;
mod rusant_main_window;
mod sender;
mod rusant_greeter;
mod rusant_login;
mod rusant_register;

use log::info;
use rusant_main_window::MainWindow;

use config::Config;
use glib::clone;
use gtk::{
    gdk::Display, glib, prelude::ActionMapExt, prelude::GtkApplicationExt, prelude::GtkWindowExt,
    CssProvider, StyleContext,
};
use gtk_macros::action;
use std::collections::HashMap;
use std::path::Path;

use gtk::gio::resources_register_include;

use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, HeaderBar, WindowTitle,
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

    provider.load_from_resource("/com/jakobwaibel/Rusant/style.css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Initialize application
    let app = Application::builder()
        .application_id(app_id)
        .resource_base_path("/com/jakobwaibel/Rusant")
        .build();

    // Run application
    app.connect_activate(build_ui);

    let actions = gio::SimpleActionGroup::new();

    app.set_action_group(Some(&actions));

    setup_accels(&app);
    {
        action! {
            actions,
            "about",
            clone!(@weak app as app => move |_, _| {
            show_about(&app);
            })
        };

        action! {
            actions,
            "show-preferences",
            clone!(@weak app as app => move |_, _| {
                show_preferences(&app);
            })
        }
    }

    info!("Starting application");

    std::process::exit(app.run());
}

/// Build the user interface
fn build_ui(app: &Application) {
    let content = libadwaita::gtk::Box::new(Orientation::Vertical, 0);

    content.append(
        &HeaderBar::builder()
            .title_widget(&WindowTitle::new("Rusant", ""))
            .build(),
    );

    let window = MainWindow::new(app);

    info!("Building UI");

    window.show();
}

/// Show the about page
fn show_about(app: &Application) {
    let window = app.active_window().unwrap();

    let dialog = libadwaita::AboutWindow::builder()
        .transient_for(&window)
        .application_name("Rusant")
        .developer_name("Jakob Waibel")
        .version("0.0.1")
        .developers(vec!["Jakob Waibel".into(), "Felicitas Pojtinger".into()])
        .copyright("© 2022 Jakob Waibel")
        .website("https://github.com/JakWai01/rusant")
        .issue_url("https://github.com/JakWai01/rusant/issues/new")
        .license_type(gtk::License::Agpl30)
        .build();

    info!("Showing about page");

    dialog.present();
}

/// Show the preferences page
fn show_preferences(app: &Application) {
    let window = app.active_window().unwrap();

    let dialog = libadwaita::PreferencesWindow::builder()
        .transient_for(&window)
        .build();

    info!("Showing preferences");

    dialog.present();
}

/// Setup keyboard shortcuts
fn setup_accels(app: &Application) {
    app.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
}
