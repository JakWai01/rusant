use gst::prelude::*;

use gtk::prelude::*;
use gtk::{gdk, gio, glib, Button, ToggleButton, Picture};

use config::Config;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::collections::HashMap;
use glib::clone;

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

    // Initialize GTK
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    // Initialize Gstreamer
    gst::init().expect("Failed to initialize Gstreamer.");

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Apply configuration
    glib::set_application_name(name);
    gtk::Window::set_default_icon_name(app_id);

    let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

    app.connect_activate(build_ui);
    app.run();

    unsafe {
        gst::deinit();
    }
}

fn build_ui(app: &gtk::Application) {
    let pipeline = gst::Pipeline::default();

    let src = gst::ElementFactory::make("autovideosrc").build().unwrap();
    let converter = gst::ElementFactory::make("videoconvert").build().unwrap();
    let sink = gst::ElementFactory::make("gtk4paintablesink")
        .build()
        .unwrap();

    let paintable = sink.property::<gdk::Paintable>("paintable");

    pipeline.add_many(&[&src, &converter, &sink]).unwrap();
    src.link_filtered(
        &converter,
        &gst_video::VideoCapsBuilder::new()
            .width(640)
            .height(480)
            .build(),
    )
    .unwrap();
    
    converter.link(&sink).unwrap();

    let window = gtk::ApplicationWindow::new(app);
    window.set_default_size(1280, 720);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let picture = gtk::Picture::new();

    picture.set_paintable(Some(&paintable));
    
    let bus = pipeline.bus().unwrap();

    let button = Button::builder()
        .label("Turn video off")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    let mut hidden = Rc::new(Cell::new(false));

    vbox.append(&picture);
    vbox.append(&button);

    button.connect_clicked(move |button| {
        if !hidden.get() {
            button.set_label("Turn video on");
            picture.hide();
            hidden.set(true);
        } else {
            button.set_label("Turn video off");
            picture.show();
            hidden.set(false);
        }
    });

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");


    window.set_child(Some(&vbox));
    window.show();

    app.add_window(&window);

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