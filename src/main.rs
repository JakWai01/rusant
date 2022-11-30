use gst::prelude::*;
use gst_app::*;
use gst_audio::*;

// use gtk::prelude::*;
// use gtk::{gdk, gio, glib, Button};

// use config::Config;
// use std::cell::{Cell, RefCell};
// use std::rc::Rc;
// use std::collections::HashMap;

// fn main() {
//     // Parse config
//     let config = Config::builder()
//         .add_source(config::File::with_name("Config"))
//         .build()
//         .unwrap()
//         .try_deserialize::<HashMap<String, String>>()
//         .unwrap();

//     let name = &config.get("name").unwrap();
//     let app_id = &config.get("app_id").unwrap();

//     // Initialize GTK
//     gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

//     // Initialize Gstreamer
//     gst::init().expect("Failed to initialize Gstreamer.");

//     gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

//     // Apply configuration
//     glib::set_application_name(name);
//     gtk::Window::set_default_icon_name(app_id);

//     let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

//     app.connect_activate(build_ui);
//     app.run();

//     unsafe {
//         gst::deinit();
//     }
// }

// fn build_ui(app: &gtk::Application) {
//     let pipeline = gst::Pipeline::default();

//     let src = gst::ElementFactory::make("autovideosrc").build().unwrap();
//     let converter = gst::ElementFactory::make("videoconvert").build().unwrap();
//     let sink = gst::ElementFactory::make("gtk4paintablesink")
//         .build()
//         .unwrap();
//     // let sink = gst::ElementFactory::make("gtkwaylandsink").build().unwrap();

//     let paintable = sink.property::<gdk::Paintable>("paintable");

//     pipeline.add_many(&[&src, &converter, &sink]).unwrap();
//     src.link_filtered(
//         &converter,
//         &gst_video::VideoCapsBuilder::new()
//             .width(640)
//             .height(480)
//             .build(),
//     )
//     .unwrap();
    
//     converter.link(&sink).unwrap();

//     let window = gtk::ApplicationWindow::new(app);
//     window.set_default_size(1280, 720);

//     let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
//     let picture = gtk::Picture::new();

//     picture.set_paintable(Some(&paintable));
    
//     let bus = pipeline.bus().unwrap();

//     let button = Button::builder()
//         .label("Turn video off")
//         .margin_top(12)
//         .margin_bottom(12)
//         .margin_start(12)
//         .margin_end(12)
//         .build();
    
//     let hidden = Rc::new(Cell::new(false));

//     vbox.append(&picture);
//     vbox.append(&button);

//     button.connect_clicked(move |button| {
//         if !hidden.get() {
//             button.set_label("Turn video on");
//             picture.hide();
//             hidden.set(true);
//         } else {
//             button.set_label("Turn video off");
//             picture.show();
//             hidden.set(false);
//         }
//     });

//     pipeline
//         .set_state(gst::State::Playing)
//         .expect("Unable to set the pipeline to the `Playing` state");


//     window.set_child(Some(&vbox));
//     window.show();

//     app.add_window(&window);

//     let app_weak = app.downgrade();
//     bus.add_watch_local(move |_, msg| {
//         use gst::MessageView;

//         let app = match app_weak.upgrade() {
//             Some(app) => app,
//             None => return glib::Continue(false),
//         };

//         match msg.view() {
//             MessageView::Eos(..) => app.quit(),
//             MessageView::Error(err) => {
//                 println!(
//                     "Error from {:?}: {} ({:?})",
//                     err.src().map(|s| s.path_string()),
//                     err.error(),
//                     err.debug()
//                 );
//                 app.quit();
//             }
//             _ => (),
//         };

//         glib::Continue(true)
//     })
//     .expect("Failed to add bus watch");

//     let pipeline = RefCell::new(Some(pipeline));
//     app.connect_shutdown(move |_| {
//         window.close();

//         if let Some(pipeline) = pipeline.borrow_mut().take() {
//             pipeline
//                 .set_state(gst::State::Null)
//                 .expect("Unable to set the pipeline to the `Null` state");
//             pipeline.bus().unwrap().remove_watch().unwrap();
//         }
//     });
// }

use glib::source::SourceId;
use std::sync::{Arc, Mutex};
use byte_slice_cast::*;

const SAMPLE_RATE: u32 = 44_100; // Samples per second we are sending
const CHUNK_SIZE: usize = 1024; // Amount of bytes we are sending in each buffer

#[derive(Debug)]
struct CustomData {
    source_id: Option<SourceId>,
    num_samples: u64, // Number of samples generated so far (for timestamp generation)
    // For waveform generation
    a: f64,
    b: f64,
    c: f64,
    d: f64,

    source: AppSrc,
    sink: AppSink,
}

impl CustomData {
    fn new(source: &AppSrc, sink: &AppSink) -> CustomData {
        CustomData {
            source_id: None,
            num_samples: 0,
            a: 0.0,
            b: 1.0,
            c: 0.0,
            d: 1.0,
            source: source.clone(),
            sink: sink.clone(),
        }
    }
}

fn main() {
    gst::init().unwrap();

    let source = gst::ElementFactory::make("appsrc").build().unwrap();
    let queue = gst::ElementFactory::make("queue").build().unwrap();
    let sink = gst::ElementFactory::make("appsink").build().unwrap();

    let pipeline = gst::Pipeline::default();

    pipeline.add_many(&[&source, &queue, &sink]).unwrap();

    gst::Element::link_many(&[&source, &queue, &sink]).unwrap();

    let info = AudioInfo::builder(gst_app::AUDIO_FORMAT_S16, SAMPLE_RATE, 1)
        .build()
        .unwrap();
    let audio_caps = info.to_caps().unwrap();

    let source = source
        .dynamic_cast::<AppSrc>()
        .expect("Source element is expected to be an appsrc!");
    source.set_caps(Some(&audio_caps));
    source.set_format(gst::Format::Time);

    let sink = sink
        .dynamic_cast::<AppSink>()
        .expect("Sink element is expected to be an appsink!");

    let data: Arc<Mutex<CustomData>> = Arc::new(Mutex::new(CustomData::new(&source, &sink)));

    let data_weak = Arc::downgrade(&data);
    let data_weak2 = Arc::downgrade(&data);

    source.set_callbacks(
        app_src::AppSrcCallbacks::builder()
            .need_data(move |_, _size| {
                let data = match data_weak.upgrade() {
                    Some(data) => data,
                    None => return,
                };

                let mut d = data.lock().unwrap();

                if d.source_id.is_none() {
                    println!("start feeding");

                    let data_weak = Arc::downgrade(&data);
                    d.source_id = Some(glib::source::idle_add(move || {
                        let data = match data_weak.upgrade() {
                            Some(data) => data,
                            None => return glib::Continue(false),
                        };

                        let (source, buffer) = {
                            let mut data = data.lock().unwrap();
                            let mut buffer = gst::Buffer::with_size(CHUNK_SIZE).unwrap();
                            let num_samples = CHUNK_SIZE / 2;

                            let pts = gst::ClockTime::SECOND
                                .mul_div_floor(data.num_samples, u64::from(SAMPLE_RATE))
                                .expect("u64 overflow");
                            let duration = gst::ClockTime::SECOND
                                .mul_div_floor(num_samples as u64, u64::from(SAMPLE_RATE))
                                .expect("u64 overflow");
                            
                            {
                                let buffer = buffer.get_mut().unwrap();
                                {
                                    let mut samples = buffer.map_writable().unwrap();
                                    let samples = samples.as_mut_slice_of::<i16>().unwrap();

                                    // Generate some psychodelic waveforms
                                    data.c += data.d;
                                    data.d -= data.c / 1000.0;
                                    let freq = 1100.0 + 1000.0 * data.d;

                                    for sample in samples.iter_mut() {
                                        data.a += data.b;
                                        data.b -= data.a / freq;
                                        *sample = 500 * (data.a as i16);
                                    }

                                    data.num_samples += num_samples as u64;
                                }

                                buffer.set_pts(pts);
                                buffer.set_duration(duration);
                            }

                            (data.source.clone(), buffer)
                        };

                        glib::Continue(source.push_buffer(buffer).is_ok())
                    }))
                }
            })
            .enough_data(move |_| {
                let data = match data_weak2.upgrade() {
                    Some(data) => data,
                    None => return,
                };

                let mut data = data.lock().unwrap();
                if let Some(source) = data.source_id.take() {
                    println!("Stop feeding");
                    source.remove();
                }
            })
            .build()
    );

    sink.set_caps(Some(&audio_caps));

    let data_weak = Arc::downgrade(&data);
    sink.set_callbacks(
        app_sink::AppSinkCallbacks::builder().new_sample(move |_| {
            let data = match data_weak.upgrade() {
                Some(data) => data,
                None => return Ok(gst::FlowSuccess::Ok),
            };

            let sink = {
                let data = data.lock().unwrap();
                data.sink.clone()
            };

            if let Ok(sample) = sink.pull_sample() {
                use std::io::{self, Write};
                print!("*");
                let _ = io::stdout().flush();
            }

            Ok(gst::FlowSuccess::Ok)
        }).build(),
    );

    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();
    let bus = pipeline.bus().unwrap();
    #[allow(clippy::single_match)]
    bus.connect_message(Some("error"), move |_, msg| match msg.view() {
        gst::MessageView::Error(err) => {
            let main_loop = &main_loop_clone;
            eprintln!(
                "Error received from element {:?}: {}",
                err.src().map(|s| s.path_string()),
                err.error()
            );
            eprintln!("Debugging information: {:?}", err.debug());
            main_loop.quit();
        }
        _ => unreachable!(),
    });

    bus.add_signal_watch();

    pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state.");

    main_loop.run();

    pipeline.set_state(gst::State::Null).expect("Unable to set the pipeline to the `Null` state.");

    bus.remove_signal_watch();
}
