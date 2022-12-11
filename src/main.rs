mod receiver;
mod sender;
mod ports;

use sender::Sender;
use receiver::*;

use gst::prelude::*;
use gtk::prelude::*;
use config::Config;
use gtk::glib;
use gtk::gio;
use std::collections::HashMap;
use std::path::Path;
use std::cell::RefCell;
use std::thread;

fn main() {
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

    // Apply configuration
    glib::set_application_name(name);
    gtk::Window::set_default_icon_name(app_id);

    gtk::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    {
        let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

        app.connect_activate(build_ui);
        app.run();
    }

    unsafe {
        gst::deinit();
    }
}

fn build_ui(app: &gtk::Application) {
    thread::spawn(|| {
        init_sender();
    });

    init_receiver(app);
}

fn init_receiver(app: &gtk::Application) {
    let (pipeline, paintable) = ReceiverPipeline::new("127.0.0.1", 5200).build();

    let window = gtk::ApplicationWindow::new(app);

    window.set_default_size(1280, 720);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let picture = gtk::Picture::new();

    picture.set_paintable(Some(&paintable));
    vbox.append(&picture);

    window.set_child(Some(&vbox));
    window.show();

    app.add_window(&window);

    let bus = pipeline.bus().unwrap();

    // Start pipeline
    pipeline.set_state(gst::State::Playing).unwrap();

    let app_weak = app.downgrade();

    bus.add_watch_local(move |_, msg| {
        use gst::MessageView;

        let app = match app_weak.upgrade() {
            Some(app) => app,
            None => return glib::Continue(false),
        };

        match msg.view() {
            MessageView::Eos(..) => app.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                app.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    })
    .expect("Failed to add bus watch");

    let pipeline = RefCell::new(Some(pipeline));
    app.connect_shutdown(move |_| {
        window.close();

        if let Some(pipeline) = pipeline.borrow_mut().take() {
            pipeline
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
            pipeline.bus().unwrap().remove_watch().unwrap();
        }
    });
}

fn init_sender() {
    let sender = sender::SenderPipeline::new("127.0.0.1", 5200);
    sender.send();
}