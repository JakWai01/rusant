use gst::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use std::cell::RefCell;

pub trait Receiver {
    fn receive(&self) {}
}

pub struct ReceiverPipeline {}

impl Receiver for ReceiverPipeline {
    fn receive(&self) {
        println!("Client: Hello!");

        gtk::init().unwrap();

        gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

        {
            let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

            app.connect_activate(build_ui);
            app.run();
        }
    }
}

fn build_ui(app: &gtk::Application) {
    // Initialize pipeline
    let pipeline = gst::Pipeline::new(None);

    // Initialize pads
    let src = gst::ElementFactory::make("udpsrc")
        .property("address", "127.0.0.1")
        .property("port", 5200)
        .build()
        .unwrap();
    let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
    let rtpjpegdepay = gst::ElementFactory::make("rtpjpegdepay").build().unwrap();
    let jpegdec = gst::ElementFactory::make("jpegdec").build().unwrap();
    let sink = gst::ElementFactory::make("gtk4paintablesink").build().unwrap();

    let caps = gst::Caps::new_simple(
        "application/x-rtp",
        &[("encoding-name", &"JPEG"), ("payload", &26i32)],
    );
    filter.set_property("caps", &caps);

    let paintable = sink.property::<gdk::Paintable>("paintable");

    // Add pads
    pipeline
        .add_many(&[&src, &filter, &rtpjpegdepay, &jpegdec, &sink])
        .unwrap();

    // Link pads
    gst::Element::link_many(&[&src, &filter, &rtpjpegdepay, &jpegdec, &sink]).unwrap();

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
