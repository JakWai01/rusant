mod feed_list;
mod feed_item;
mod article_list;
mod article_item;
mod main_window;
mod call_window;
mod ports;
mod receiver;
mod sender;

// use main_window::MainWindow;
use call_window::CallWindow;

use receiver::*;
use sender::Sender;

use config::Config;
use gst::prelude::*;
use gtk::{glib, traits::GtkWindowExt, CssProvider, StyleContext, gdk::Display};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::thread;

use gtk::{
    gio::resources_register_include,
};

use gio::*;

use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, ApplicationWindow, HeaderBar, WindowTitle,
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

    // Initialize Gstreamer
    gst::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Initialize variables
    glib::set_application_name(name);
    gtk::Window::set_default_icon_name(app_id);

    gtk::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    resources_register_include!("gtk-rusant.gresource").expect("Failed to register resources.");
    
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    // provider.load_from_data(include_bytes!("../content/style.css"));
    provider.load_from_resource("./style.css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let app = Application::builder()
        .application_id("com.jakobwaibel.rusant")
        // .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();

    app.connect_activate(build_ui);
    app.run();

    // Image from_gicon
    // let i: Icon = gio::Icon::for_string("Jakob Waibel").unwrap();

    unsafe {
        gst::deinit();
    }
}

fn build_ui(app: &Application) {
    // let sender_pipeline = sender::SenderPipeline::new("127.0.0.1", 5200);
    // thread::spawn(move || {
    //     sender_pipeline.send();
    // });

    // let (receiver_pipeline, _receiver_paintable) = ReceiverPipeline::new("127.0.0.1", 5200).build();

    let content = libadwaita::gtk::Box::new(Orientation::Vertical, 0);

    content.append(
        &HeaderBar::builder()
            .title_widget(&WindowTitle::new("Rusant", ""))
            .build(),
    );

    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("Rusant")
    //     .default_height(720)
    //     .default_width(1280)
    //     .content(&content)
    //     .build();

    // let window = MainWindow::new(app);
    let window = CallWindow::new(app);

    // Can we just get the paintable element and call the correct method? 

    // window.set_default_height(720);
    // window.set_default_width(1280);

    // let picture = gtk::Picture::new();
    // picture.set_paintable(Some(&receiver_paintable));

    // let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    // vbox.append(&picture);

    // window.set_child(Some(&vbox));
    window.show();

    // app.add_window(&window);

    // let bus = receiver_pipeline.bus().unwrap();

    // // Start pipeline
    // receiver_pipeline.set_state(gst::State::Playing).unwrap();

    // let app_weak = app.downgrade();

    // bus.add_watch_local(move |_, msg| {
    //     use gst::MessageView;

    //     let app = match app_weak.upgrade() {
    //         Some(app) => app,
    //         None => return glib::Continue(false),
    //     };

    //     match msg.view() {
    //         MessageView::Eos(..) => app.quit(),
    //         MessageView::Error(err) => {
    //             println!(
    //                 "Error from {:?}: {} ({:?})",
    //                 err.src().map(|s| s.path_string()),
    //                 err.error(),
    //                 err.debug()
    //             );
    //             app.quit();
    //         }
    //         _ => (),
    //     };

    //     glib::Continue(true)
    // })
    // .expect("Failed to add bus watch");

    // let receiver_pipeline = RefCell::new(Some(receiver_pipeline));
    // app.connect_shutdown(move |_| {
    //     // window.close();

    //     if let Some(receiver_pipeline) = receiver_pipeline.borrow_mut().take() {
    //         receiver_pipeline
    //             .set_state(gst::State::Null)
    //             .expect("Unable to set the pipeline to the `Null` state");
    //         receiver_pipeline.bus().unwrap().remove_watch().unwrap();
    //     }
    // });
}