use crate::{receiver, sender::{self, Sender}};

use super::CallPane;

use std::thread;

use libadwaita::{HeaderBar, StatusPage};
use anyhow::Error;
use derive_more::{Display, Error};
use glib::{
    self, clone, object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    ObjectExt,
};

use gst::{prelude::GstBinExtManual, traits::ElementExt, prelude::*, element_warning, element_error};

use gtk::{
    gdk,
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    traits::{ButtonExt, WidgetExt},
    ActionBar, Box, Button, CompositeTemplate, FlowBox,
};
use log::info;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-call-pane.ui")]
pub struct CallPaneTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub back_button: TemplateChild<Button>,

    #[template_child]
    pub grid: TemplateChild<FlowBox>,

    #[template_child]
    pub placeholder: TemplateChild<StatusPage>,

    #[template_child]
    pub call_box: TemplateChild<Box>,

    #[template_child]
    pub action_bar: TemplateChild<ActionBar>,

    #[template_child]
    pub camera_video: TemplateChild<Button>,

    #[template_child]
    pub audio_input_microphone: TemplateChild<Button>,

    #[template_child]
    pub call_stop: TemplateChild<Button>,
}

#[object_subclass]
impl ObjectSubclass for CallPaneTemplate {
    const NAME: &'static str = "CallPane";

    type Type = CallPane;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CallPaneTemplate {
    /// Construct a new CallPane
    fn constructed(&self) {
        self.parent_constructed();

        // Handle click on camera_video button
        self.camera_video.connect_clicked(move |button| {
            info!("Button `camera_video` was clicked");

            let css_class = "suggested-action";

            // Check if the button currently has the `suggested-action` css class
            if button.has_css_class(css_class) {
                button.remove_css_class(css_class);
            } else {
                button.add_css_class(css_class);
            }
        });

        // Handle click on audio_input_microphone button
        self.audio_input_microphone.connect_clicked(move |button| {
            info!("Button `audio_input_microphone was clicked");

            let css_class = "suggested-action";

            // Check if button currently has the `suggested-action` css class
            if button.has_css_class(css_class) {
                button.remove_css_class(css_class);
            } else {
                button.add_css_class(css_class);
            }
        });

        // Handle click on call_stop button
        self.call_stop
            .connect_clicked(clone!(@weak self as this => move |_| {
                info!("Button `call_stop` was clicked");

                // Hide call and show placeholder
                this.placeholder.set_visible(true);
                this.action_bar.set_visible(false);
                this.call_box.set_visible(false);
            }));

        // Initialize testing gstreamer pipeline - This section should be removed later on
        // let pipeline = gst::Pipeline::default();

        // let src = gst::ElementFactory::make("v4l2src").build().unwrap();

        // src.set_property("device", "/dev/video0");

        // let caps = gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        // let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        // filter.set_property("caps", &caps);

        // let convert = gst::ElementFactory::make("videoconvert").build().unwrap();

        // let sink = gst::ElementFactory::make("gtk4paintablesink")
        //     .build()
        //     .unwrap();

        // let paintable = sink.property::<gdk::Paintable>("paintable");
        // pipeline
        //     .add_many(&[&src, &filter, &convert, &sink])
        //     .unwrap();

        // gst::Element::link_many(&[&src, &filter, &convert, &sink]).unwrap();

        // let picture = gtk::Picture::new();
        // picture.set_paintable(Some(&paintable));
        // picture.set_keep_aspect_ratio(true);

        // let pipeline_test = gst::Pipeline::default();

        // let src_test = gst::ElementFactory::make("videotestsrc").build().unwrap();

        // let caps_test =
        //     gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        // let filter_test = gst::ElementFactory::make("capsfilter").build().unwrap();
        // filter_test.set_property("caps", &caps_test);

        // let convert_test = gst::ElementFactory::make("videoconvert").build().unwrap();

        // let sink_test = gst::ElementFactory::make("gtk4paintablesink")
        //     .build()
        //     .unwrap();

        // let paintable_test = sink_test.property::<gdk::Paintable>("paintable");
        // pipeline_test
        //     .add_many(&[&src_test, &filter_test, &convert_test, &sink_test])
        //     .unwrap();

        // gst::Element::link_many(&[&src_test, &filter_test, &convert_test, &sink_test]).unwrap();

        // let picture_test = gtk::Picture::new();
        // picture_test.set_paintable(Some(&paintable_test));
        // picture_test.set_keep_aspect_ratio(true);

        // let pipeline_demo = gst::Pipeline::default();

        // let src_demo = gst::ElementFactory::make("videotestsrc").build().unwrap();

        // let caps_demo =
        //     gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        // let filter_demo = gst::ElementFactory::make("capsfilter").build().unwrap();
        // filter_demo.set_property("caps", &caps_demo);

        // let convert_demo = gst::ElementFactory::make("videoconvert").build().unwrap();

        // let sink_demo = gst::ElementFactory::make("gtk4paintablesink")
        //     .build()
        //     .unwrap();

        // let paintable_demo = sink_demo.property::<gdk::Paintable>("paintable");
        // pipeline_demo
        //     .add_many(&[&src_demo, &filter_demo, &convert_demo, &sink_demo])
        //     .unwrap();

        // gst::Element::link_many(&[&src_demo, &filter_demo, &convert_demo, &sink_demo]).unwrap();

        // let picture_demo = gtk::Picture::new();
        // picture_demo.set_paintable(Some(&paintable_demo));
        // picture_demo.set_keep_aspect_ratio(true);

        // self.grid.insert(&picture, 0);
        // self.grid.insert(&picture_test, 1);
        // // self.grid.insert(&picture_demo, 2);

        // thread::spawn(move || {
        //     pipeline
        //         .set_state(gst::State::Playing)
        //         .expect("Unable to set the pipeline to the `Playing` state");
        // });
        // thread::spawn(move || {
        //     pipeline_test
        //         .set_state(gst::State::Playing)
        //         .expect("Unable to set the pipeline to the `Playing` state");
        // });
        // thread::spawn(move || {
        //     pipeline_demo
        //         .set_state(gst::State::Playing)
        //         .expect("Unable to set the pipeline to the `Playing` state");
        // });

    /*
     * This part does not necessarily need to be here.
     * It just has to be started once a call starts but this can be anywhere. 
     */
    let sender = sender::SenderPipeline::new("127.0.0.1", 3000);
    sender.build();

    thread::spawn(move || {
        sender.send();
    });

    let receiver = receiver::ReceiverPipeline::new("127.0.0.1", 3000);
    let (pipeline, paintable) = receiver.build();

    let picture = gtk::Picture::new();
    picture.set_paintable(Some(&paintable));
    picture.set_keep_aspect_ratio(true);

    self.grid.insert(&picture, 0);

    thread::spawn(move || {
        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");
    });

    // Create the pipeline for the sound serving on port 3001

    // Initialize Gstreamer pipeline
    let pipeline_audio = gst::Pipeline::new(Some("Sound"));

    // Initialize pads
    let src = gst::ElementFactory::make("alsasrc").build().unwrap();
    let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
    let vorbisenc = gst::ElementFactory::make("vorbisenc").build().unwrap();
    let rtpvorbispay = gst::ElementFactory::make("rtpvorbispay").build().unwrap();
    let rtpstreampay = gst::ElementFactory::make("rtpstreampay").build().unwrap();
    let sink = gst::ElementFactory::make("tcpserversink").build().unwrap();

    // Initialize caps
    let caps = gst::Caps::new_simple("audio/x-raw", &[("rate", &48000i32)]);

    filter.set_property("caps", &caps);

    rtpvorbispay.set_property("config-interval", 1u32);
    
    sink.set_property("host", "127.0.0.1");
    sink.set_property("port", 3001);

    pipeline_audio.add_many(&[&src, &filter, &vorbisenc, &rtpvorbispay, &rtpstreampay, &sink]).unwrap();
    
    gst::Element::link_many(&[&src, &filter, &vorbisenc, &rtpvorbispay, &rtpstreampay, &sink]).unwrap();
    
    thread::spawn(move || {
        pipeline_audio.set_state(gst::State::Playing).expect("Unable to set the audio pipeline to the `Playing` state");
    });


    // Create pipeline for listening to the sound 

    // Initializing Gstreamer pipeline
    let pipeline_listening = gst::Pipeline::new(Some("Audio"));

    // Initialize pads
    let src_l = gst::ElementFactory::make("tcpclientsrc").build().unwrap();
    let filter_l = gst::ElementFactory::make("capsfilter").build().unwrap();
    let rtpstreamdepay = gst::ElementFactory::make("rtpstreamdepay").build().unwrap();
    let rtpvorbisdepay = gst::ElementFactory::make("rtpvorbisdepay").build().unwrap();
    let decodebin = gst::ElementFactory::make("decodebin").build().unwrap();
    // let filter_fix = gst::ElementFactory::make("capsfilter").build().unwrap();
    // let audioconvert = gst::ElementFactory::make("audioconvert").build().unwrap();
    // let audioresample = gst::ElementFactory::make("audioresample").build().unwrap();
    // let autoaudiosink = gst::ElementFactory::make("autoaudiosink").build().unwrap();
    
    src_l.set_property("host", "127.0.0.1");
    src_l.set_property("port", 3001);
    src_l.set_property("do-timestamp", true);

    let caps_l = gst::Caps::new_simple("application/x-rtp-stream", &[("media", &"audio"), ("clock-rate", &48000i32), ("encoding-name", &"VORBIS")]);

    filter_l.set_property("caps", &caps_l);

    pipeline_listening.add_many(&[&src_l, &filter_l, &rtpstreamdepay, &rtpvorbisdepay, &decodebin]).unwrap();

    gst::Element::link_many(&[&src_l, &filter_l, &rtpstreamdepay, &rtpvorbisdepay, &decodebin]).unwrap();

    let pipeline_weak = pipeline_listening.downgrade();

    decodebin.connect_pad_added(move |dbin, src_pad| {
        // Here we temporarily retrieve a strong reference on the pipeline from the weak one
        // we moved into this callback.
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return,
        };

        // Try to detect whether the raw stream decodebin provided us with
        // just now is either audio or video (or none of both, e.g. subtitles).
        let (is_audio, is_video) = {
            let media_type = src_pad.current_caps().and_then(|caps| {
                caps.structure(0).map(|s| {
                    let name = s.name();
                    (name.starts_with("audio/"), name.starts_with("video/"))
                })
            });

            match media_type {
                None => {
                    element_warning!(
                        dbin,
                        gst::CoreError::Negotiation,
                        ("Failed to get media type from pad {}", src_pad.name())
                    );

                    return;
                }
                Some(media_type) => media_type,
            }
        };

        // We create a closure here, calling it directly below it, because this greatly
        // improves readability for error-handling. Like this, we can simply use the
        // ?-operator within the closure, and handle the actual error down below where
        // we call the insert_sink(..) closure.
        let insert_sink = |is_audio, is_video| -> Result<(), Error> {
            if is_audio {
                // decodebin found a raw audiostream, so we build the follow-up pipeline to
                // play it on the default audio playback device (using autoaudiosink).
                let queue = gst::ElementFactory::make("queue").build().unwrap();
                let convert = gst::ElementFactory::make("audioconvert").build().unwrap();
                let resample = gst::ElementFactory::make("audioresample").build().unwrap();
                let sink = gst::ElementFactory::make("autoaudiosink").build().unwrap();
                
                let elements = &[&queue, &convert, &resample, &sink];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                // !!ATTENTION!!:
                // This is quite important and people forget it often. Without making sure that
                // the new elements have the same state as the pipeline, things will fail later.
                // They would still be in Null state and can't process data.
                for e in elements {
                    e.sync_state_with_parent()?;
                }

                // Get the queue element's sink pad and link the decodebin's newly created
                // src pad for the audio stream to it.
                let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                src_pad.link(&sink_pad)?;
            } else if is_video {
                // decodebin found a raw videostream, so we build the follow-up pipeline to
                // display it using the autovideosink.
                let queue = gst::ElementFactory::make("queue").build().unwrap();
                let convert = gst::ElementFactory::make("videoconvert").build().unwrap();
                let scale = gst::ElementFactory::make("videoscale").build().unwrap();
                let sink = gst::ElementFactory::make("autovideosink").build().unwrap();

                let elements = &[&queue, &convert, &scale, &sink];
                pipeline.add_many(elements)?;
                gst::Element::link_many(elements)?;

                for e in elements {
                    e.sync_state_with_parent()?
                }

                // Get the queue element's sink pad and link the decodebin's newly created
                // src pad for the video stream to it.
                let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                src_pad.link(&sink_pad)?;
            }

            Ok(())
        };

        // When adding and linking new elements in a callback fails, error information is often sparse.
        // GStreamer's built-in debugging can be hard to link back to the exact position within the code
        // that failed. Since callbacks are called from random threads within the pipeline, it can get hard
        // to get good error information. The macros used in the following can solve that. With the use
        // of those, one can send arbitrary rust types (using the pipeline's bus) into the mainloop.
        // What we send here is unpacked down below, in the iteration-code over sent bus-messages.
        // Because we are using the failure crate for error details here, we even get a backtrace for
        // where the error was constructed. (If RUST_BACKTRACE=1 is set)
        if let Err(err) = insert_sink(is_audio, is_video) {
            // The following sends a message of type Error on the bus, containing our detailed
            // error information.
            #[cfg(feature = "v1_10")]
            element_error!(
                dbin,
                gst::LibraryError::Failed,
                ("Failed to insert sink"),
                details: gst::Structure::builder("error-details")
                            .field("error",
                                   &ErrorValue(Arc::new(Mutex::new(Some(err)))))
                            .build()
            );

            #[cfg(not(feature = "v1_10"))]
            element_error!(
                dbin,
                gst::LibraryError::Failed,
                ("Failed to insert sink"),
                ["{}", err]
            );
        }
    });

    thread::spawn(move || {
        pipeline_listening.set_state(gst::State::Playing).expect("Unable to set the audio pipeline to the `Playing` state");
    });
    }
}

impl WidgetImpl for CallPaneTemplate {}
impl BoxImpl for CallPaneTemplate {}
