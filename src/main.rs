use gst::prelude::*;

use gtk::prelude::*;
use gtk::{gdk, gio, glib};

use std::cell::RefCell;

use std::path::Path;
use std::process;

// const gst_src: &str = "v4l2src device=";
// const gst_src_format: &str = "video/x-raw,format=RGB";
// const gst_videosink: &str = "ximagesink";
// const sep: &str = " ! ";

// gst-launch-1.0 v4l2src name=cam_src ! decodebin ! videoconvert ! videoscale ! video/x-raw,format=RGB ! queue ! videoconvert ! ximagesink name=img_origin
fn main() {
    if !Path::new("/dev/video0").exists() {
        panic!("No webcam detected: /dev/video0 cannot be found.")
    }

    gtk::init().unwrap();
    gst::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    {
        let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

        app.connect_activate(build_ui);
        app.run();
    }

    unsafe {
        gst::deinit();
    }

    // let src: String = gst_src.to_string() + "/dev/video0";
    // let src_format: String = gst_src_format.to_string(); // + ",width=1920,height=1080,framerate=60";
    // let videosink = gst_videosink;
    // let video_pipeline = src + sep + &src_format + sep + videosink;

    // println!("{}", video_pipeline);

    // let command_pipeline = "v4l2src name=cam_src ! decodebin ! videoconvert ! videoscale ! video/x-raw,format=RGB ! queue ! videoconvert ! ximagesink name=img_origin";
    // let pipeline = gst::parse_launch(command_pipeline).unwrap();
    // pipeline
    //     .set_state(gst::State::Playing)
    //     .expect("Unable to set the pipeline to the `Playing` state");

    // let bus = pipeline.bus().unwrap();
    // for msg in bus.iter_timed(gst::ClockTime::NONE) {
    //     use gst::MessageView;

    //     match msg.view() {
    //         MessageView::Eos(..) => break,
    //         MessageView::Error(err) => {
    //             println!(
    //                 "Error from {:?}: {} ({:?})",
    //                 err.src().map(|s| s.path_string()),
    //                 err.error(),
    //                 err.debug()
    //             );
    //             break;
    //         }
    //         _ => (),
    //     }
    // }

    // pipeline
    //     .set_state(gst::State::Null)
    //     .expect("Unable to set the pipeline to the `Null` state");

    // // GTK
    // let app = gtk::Application::builder().application_id("rusant").build();
    // app.connect_activate(build_ui);
    // app.run();

    // // Video overlay
    // let video_window: gtk::DrawingArea = gtk::DrawingArea::new();

    // let video_overlay = pipeline.clone().dynamic_cast::<gst_video::VideoOverlay>().unwrap();

    // video_window.connect_realize(move |video_window| {
    //     let video_overlay = &video_overlay;
    //     // if let Some(v) = video_window.native() {
    //     //     println!("{}", v);
    //     // };

    //     // match video_window.native() {
    //     //     Some(v) => {},
    //     //     None => {
    //     // }

    //     if let None = video_window.native() {
    //         println!("Can't create native window for widget");
    //         process::exit(-1);
    //     };

    //     let display_type_name = video_window.display().type_().name();
    //     // let gdk_window = video_window.window().unwrap();

    //     // if !gdk_window.ensure_native() {
    //     //     println!("Can't create native window for widget");
    //     //     process::exit(-1);
    //     // }

    //     // let display_type_name = gdk_window.display().type_().name();
    //     println!("Display type: {}", display_type_name);
    //     // {
    //     //     // Check if we're using X11 or ...
    //     //     if display_type_name == "GdkX11Display" {
    //     //         extern "C" {
    //     //             pub fn gdk_x11_window_get_xid (
    //     //                 window: *mut glib::object::Object,
    //     //             ) -> *mut c_void;
    //     //         }

    //     //         #[allow(clippy::cast_ptr_alignment)]
    //     //         unsafe {
    //     //             let xid = gdk_x11_window_get_xid(video_window.as_ptr() as *mut _);
    //     //             video_overlay.set_window_handle(xid as usize);
    //     //         }
    //     //     } else {
    //     //         println!("Add support for display type '{}'", display_type_name);
    //     //         process::exit(-1);
    //     //     }
    //     // }
    // });
}

fn build_ui(app: &gtk::Application) {
    //     let file = gtk::MediaFile::for_filename("video.mp4");

    //     let vid = gtk::Video::for_media_stream(Some(&file));

    //     let window = gtk::ApplicationWindow::builder()
    //         .application(app)
    //         .title("Rusant")
    //         .child(&vid)
    //         .build();

    //     window.present();
    
    // Somewhat equivalent to this: gst-launch-1.0 videotestsrc ! videoconvert ! autovideosink
    let pipeline = gst::Pipeline::default();
    let src = gst::ElementFactory::make("videotestsrc").build().unwrap();

    let overlay = gst::ElementFactory::make("clockoverlay")
        .property("font-desc", "Monospace 42")
        .build()
        .unwrap();

    let sink = gst::ElementFactory::make("gtk4paintablesink")
        .build()
        .unwrap();
    let paintable = sink.property::<gdk::Paintable>("paintable");

    pipeline.add_many(&[&src, &overlay, &sink]).unwrap();
    src.link_filtered(
        &overlay,
        &gst_video::VideoCapsBuilder::new()
            .width(640)
            .height(480)
            .build(),
    ).unwrap();
    overlay.link(&sink).unwrap();

    let window = gtk::ApplicationWindow::new(app);
    window.set_default_size(640, 480);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let picture = gtk::Picture::new();
    let label = gtk::Label::new(Some("Position: 00:00:00"));

    picture.set_paintable(Some(&paintable));
    vbox.append(&picture);
    vbox.append(&label);

    window.set_child(Some(&vbox));
    window.show();

    app.add_window(&window);

    let pipeline_weak = pipeline.downgrade();
    let timeout_id = glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return glib::Continue(true),
        };

        let position = pipeline.query_position::<gst::ClockTime>();;
        label.set_text(&format!("Position: {:.0}", position.display()));
        glib::Continue(true)
    });

    let bus = pipeline.bus().unwrap();

    pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state");
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
    }).expect("Failed to add bus watch");

    let timeout_id = RefCell::new(Some(timeout_id));
    let pipeline = RefCell::new(Some(pipeline));
    app.connect_shutdown(move |_| {
        window.close();

        if let Some(pipeline) = pipeline.borrow_mut().take() {
            pipeline.set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
            pipeline.bus().unwrap().remove_watch().unwrap();
        }

        if let Some(timeout_id) = timeout_id.borrow_mut().take() {
            timeout_id.remove();
        }
    });
}
