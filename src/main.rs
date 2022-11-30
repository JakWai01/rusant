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

// use glib::source::SourceId;
// use std::sync::{Arc, Mutex};
// use byte_slice_cast::*;

// const SAMPLE_RATE: u32 = 44_100; // Samples per second we are sending
// const CHUNK_SIZE: usize = 1024; // Amount of bytes we are sending in each buffer

// #[derive(Debug)]
// struct CustomData {
//     source_id: Option<SourceId>,
//     num_samples: u64, // Number of samples generated so far (for timestamp generation)
//     // For waveform generation
//     a: f64,
//     b: f64,
//     c: f64,
//     d: f64,

//     source: AppSrc,
//     sink: AppSink,
// }

// impl CustomData {
//     fn new(source: &AppSrc, sink: &AppSink) -> CustomData {
//         CustomData {
//             source_id: None,
//             num_samples: 0,
//             a: 0.0,
//             b: 1.0,
//             c: 0.0,
//             d: 1.0,
//             source: source.clone(),
//             sink: sink.clone(),
//         }
//     }
// }

// fn main() {
//     gst::init().unwrap();

//     let source = gst::ElementFactory::make("appsrc").build().unwrap();
//     let queue = gst::ElementFactory::make("queue").build().unwrap();
//     let sink = gst::ElementFactory::make("appsink").build().unwrap();

//     let pipeline = gst::Pipeline::default();

//     pipeline.add_many(&[&source, &queue, &sink]).unwrap();

//     gst::Element::link_many(&[&source, &queue, &sink]).unwrap();

//     let info = AudioInfo::builder(gst_app::AUDIO_FORMAT_S16, SAMPLE_RATE, 1)
//         .build()
//         .unwrap();
//     let audio_caps = info.to_caps().unwrap();

//     let source = source
//         .dynamic_cast::<AppSrc>()
//         .expect("Source element is expected to be an appsrc!");
//     source.set_caps(Some(&audio_caps));
//     source.set_format(gst::Format::Time);

//     let sink = sink
//         .dynamic_cast::<AppSink>()
//         .expect("Sink element is expected to be an appsink!");

//     let data: Arc<Mutex<CustomData>> = Arc::new(Mutex::new(CustomData::new(&source, &sink)));

//     let data_weak = Arc::downgrade(&data);
//     let data_weak2 = Arc::downgrade(&data);

//     source.set_callbacks(
//         app_src::AppSrcCallbacks::builder()
//             .need_data(move |_, _size| {
//                 let data = match data_weak.upgrade() {
//                     Some(data) => data,
//                     None => return,
//                 };

//                 let mut d = data.lock().unwrap();

//                 if d.source_id.is_none() {
//                     println!("start feeding");

//                     let data_weak = Arc::downgrade(&data);
//                     d.source_id = Some(glib::source::idle_add(move || {
//                         let data = match data_weak.upgrade() {
//                             Some(data) => data,
//                             None => return glib::Continue(false),
//                         };

//                         let (source, buffer) = {
//                             let mut data = data.lock().unwrap();
//                             let mut buffer = gst::Buffer::with_size(CHUNK_SIZE).unwrap();
//                             let num_samples = CHUNK_SIZE / 2;

//                             let pts = gst::ClockTime::SECOND
//                                 .mul_div_floor(data.num_samples, u64::from(SAMPLE_RATE))
//                                 .expect("u64 overflow");
//                             let duration = gst::ClockTime::SECOND
//                                 .mul_div_floor(num_samples as u64, u64::from(SAMPLE_RATE))
//                                 .expect("u64 overflow");
                            
//                             {
//                                 let buffer = buffer.get_mut().unwrap();
//                                 {
//                                     let mut samples = buffer.map_writable().unwrap();
//                                     let samples = samples.as_mut_slice_of::<i16>().unwrap();

//                                     // Generate some psychodelic waveforms
//                                     data.c += data.d;
//                                     data.d -= data.c / 1000.0;
//                                     let freq = 1100.0 + 1000.0 * data.d;

//                                     for sample in samples.iter_mut() {
//                                         data.a += data.b;
//                                         data.b -= data.a / freq;
//                                         *sample = 500 * (data.a as i16);
//                                     }

//                                     data.num_samples += num_samples as u64;
//                                 }

//                                 buffer.set_pts(pts);
//                                 buffer.set_duration(duration);
//                             }

//                             (data.source.clone(), buffer)
//                         };

//                         glib::Continue(source.push_buffer(buffer).is_ok())
//                     }))
//                 }
//             })
//             .enough_data(move |_| {
//                 let data = match data_weak2.upgrade() {
//                     Some(data) => data,
//                     None => return,
//                 };

//                 let mut data = data.lock().unwrap();
//                 if let Some(source) = data.source_id.take() {
//                     println!("Stop feeding");
//                     source.remove();
//                 }
//             })
//             .build()
//     );

//     sink.set_caps(Some(&audio_caps));

//     let data_weak = Arc::downgrade(&data);
//     sink.set_callbacks(
//         app_sink::AppSinkCallbacks::builder().new_sample(move |_| {
//             let data = match data_weak.upgrade() {
//                 Some(data) => data,
//                 None => return Ok(gst::FlowSuccess::Ok),
//             };

//             let sink = {
//                 let data = data.lock().unwrap();
//                 data.sink.clone()
//             };

//             if let Ok(sample) = sink.pull_sample() {
//                 use std::io::{self, Write};
//                use gst::element_error;

//                 // size = width * height * bpp
//                 println!("---------------------------------------");
//                 println!("{:?}", sample.buffer().unwrap().size());

//                 // A buffer contains mutliple memory objects. 
//                 println!("{:?}", sample.buffer().unwrap().memory(0).unwrap());

//                 println!("{:?}", sample.buffer().unwrap().peek_memory(0));

//                 println!("{:?}", sample.buffer().unwrap().map_readable().unwrap());

//                 let buffer = sample.buffer().ok_or_else(|| {
//                     element_error!(sink, gst::ResourceError::Failed, ("Failed to get buffer from appsink"));
//                     gst::FlowError::Error;
//                 });
//                 println!("---------------------------------------");
//                 let _ = io::stdout().flush();
//             }

//             Ok(gst::FlowSuccess::Ok)
//         }).build(),
//     );

//     let main_loop = glib::MainLoop::new(None, false);
//     let main_loop_clone = main_loop.clone();
//     let bus = pipeline.bus().unwrap();
//     #[allow(clippy::single_match)]
//     bus.connect_message(Some("error"), move |_, msg| match msg.view() {
//         gst::MessageView::Error(err) => {
//             let main_loop = &main_loop_clone;
//             eprintln!(
//                 "Error received from element {:?}: {}",
//                 err.src().map(|s| s.path_string()),
//                 err.error()
//             );
//             eprintln!("Debugging information: {:?}", err.debug());
//             main_loop.quit();
//         }
//         _ => unreachable!(),
//     });

//     bus.add_signal_watch();

//     pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state.");

//     main_loop.run();

//     pipeline.set_state(gst::State::Null).expect("Unable to set the pipeline to the `Null` state.");

//     bus.remove_signal_watch();
// }

use gst::element_error;
use gst::prelude::*;

use byte_slice_cast::*;

use std::i16;
use std::i32;

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init().unwrap();

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("audiotestsrc").build().unwrap();
    let sink = gst::ElementFactory::make("appsink").build().unwrap();

    pipeline.add_many(&[&src, &sink]).unwrap();
    src.link(&sink).unwrap();

    let appsink = sink
        .dynamic_cast::<gst_audio::AppSink>()
        .expect("Sink element is expected to be an appsink!");

    // Tell the appsink what format we want. It will then be the audiotestsrc's job to
    // provide the format we request.
    // This can be set after linking the two objects, because format negotiation between
    // both elements will happen during pre-rolling of the pipeline.
    appsink.set_caps(Some(
        &gst::Caps::builder("audio/x-raw")
            .field("format", gst_app::AUDIO_FORMAT_S16.to_str())
            .field("layout", "interleaved")
            .field("channels", 1i32)
            .field("rate", gst::IntRange::<i32>::new(1, i32::MAX))
            .build(),
    ));

    // Getting data out of the appsink is done by setting callbacks on it.
    // The appsink will then call those handlers, as soon as data is available.
    appsink.set_callbacks(
        app_sink::AppSinkCallbacks::builder()
            // Add a handler to the "new-sample" signal.
            .new_sample(|appsink| {
                // Pull the sample in question out of the appsink's buffer.
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;

                // At this point, buffer is only a reference to an existing memory region somewhere.
                // When we want to access its content, we have to map it while requesting the required
                // mode of access (read, read/write).
                // This type of abstraction is necessary, because the buffer in question might not be
                // on the machine's main memory itself, but rather in the GPU's memory.
                // So mapping the buffer makes the underlying memory region accessible to us.
                // See: https://gstreamer.freedesktop.org/documentation/plugin-development/advanced/allocation.html
                let map = buffer.map_readable().map_err(|_| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to map buffer readable")
                    );

                    gst::FlowError::Error
                })?;

                // We know what format the data in the memory region has, since we requested
                // it by setting the appsink's caps. So what we do here is interpret the
                // memory region we mapped as an array of signed 16 bit integers.
                let samples = map.as_slice_of::<i16>().map_err(|_| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to interprete buffer as S16 PCM")
                    );

                    gst::FlowError::Error
                })?;

                // For buffer (= chunk of samples), we calculate the root mean square:
                // (https://en.wikipedia.org/wiki/Root_mean_square)
                // let sum: f64 = samples
                //     .iter()
                //     .map(|sample| {
                //         let f = f64::from(*sample) / f64::from(i16::MAX);
                //         f * f
                //     })
                //     .sum();
                // let rms = (sum / (samples.len() as f64)).sqrt();
                // for sample in samples.iter() {
                //     println!("{:?}", sample);
                // }
                println!("{:?}", samples);
//                println!("rms: {}", rms);

                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );

    Ok(pipeline)
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null).unwrap();
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();

    Ok(())
}

fn example_main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}

fn main() {
    // tutorials_common::run is only required to set up the application environment on macOS
    // (but not necessary in normal Cocoa applications where this is set up automatically)
    example_main();
}
