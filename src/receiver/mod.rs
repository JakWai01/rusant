use anyhow::Error;
use gst::{element_error, element_warning, prelude::*};
use gtk::gdk;
use log::info;

/// Receiver part of the gstreamer pipeline
pub struct VideoReceiverPipeline<'a> {
    address: &'a str,
    port: i32,
    pipeline: gst::Pipeline,
}

pub struct AudioReceiverPipeline<'a> {
    address: &'a str,
    port: i32,
    pipeline: gst::Pipeline,
}

impl<'a> VideoReceiverPipeline<'a> {
    /// Initialize a new VideoReceiverPipeline
    pub fn new(address: &'a str, port: i32) -> Self {
        VideoReceiverPipeline {
            address,
            port,
            pipeline: gst::Pipeline::new(Some("VideoReceiver")),
        }
    }

    pub fn start(&self) {
        self.pipeline.set_state(gst::State::Playing).unwrap();
    }

    pub fn stop(&self) {
        self.pipeline.set_state(gst::State::Null).unwrap();
    }

    /// Build the pipeline
    pub fn build(&self) -> gdk::Paintable {
        info!("Initializing video receiver pipeline");

        // Initialize Gstreamer pipeline
        let pipeline = &self.pipeline;

        // Initialize pads
        let src = gst::ElementFactory::make("tcpclientsrc")
            .property("host", self.address)
            .property("port", self.port)
            .build()
            .unwrap();
        let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        let rtpstreamdepay = gst::ElementFactory::make("rtpstreamdepay").build().unwrap();
        let rtpjpegdepay = gst::ElementFactory::make("rtpjpegdepay").build().unwrap();
        let jpegdec = gst::ElementFactory::make("jpegdec").build().unwrap();
        let sink = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .unwrap();

        // Initialize Caps
        let caps = gst::Caps::new_simple("application/x-rtp-stream", &[("encoding-name", &"JPEG")]);
        filter.set_property("caps", &caps);

        // Create paintable to paint the webcam picture in
        let paintable = sink.property::<gdk::Paintable>("paintable");

        // Add pads
        pipeline
            .add_many(&[
                &src,
                &filter,
                &rtpstreamdepay,
                &rtpjpegdepay,
                &jpegdec,
                &sink,
            ])
            .unwrap();

        // Link pads
        gst::Element::link_many(&[
            &src,
            &filter,
            &rtpstreamdepay,
            &rtpjpegdepay,
            &jpegdec,
            &sink,
        ])
        .unwrap();

        paintable
    }
}

impl<'a> AudioReceiverPipeline<'a> {
    /// Initialize a new AudioReceiverPipeline
    pub fn new(address: &'a str, port: i32) -> Self {
        AudioReceiverPipeline {
            address,
            port,
            pipeline: gst::Pipeline::new(Some("AudioReceiver")),
        }
    }

    pub fn start(&self) {
        self.pipeline.set_state(gst::State::Playing).unwrap();
    }

    pub fn stop(&self) {
        self.pipeline.set_state(gst::State::Null).unwrap();
    }

    /// Build the pipeline
    pub fn build(&self) {
        info!("Initializing audio receiver pipeline");

        // Initialize Gstreamer pipeline
        let pipeline = &self.pipeline;

        // Initialize pads
        let src = gst::ElementFactory::make("tcpclientsrc").build().unwrap();
        let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        let rtpstreamdepay = gst::ElementFactory::make("rtpstreamdepay").build().unwrap();
        let rtpvorbisdepay = gst::ElementFactory::make("rtpvorbisdepay").build().unwrap();
        let decodebin = gst::ElementFactory::make("decodebin").build().unwrap();

        src.set_property("host", self.address);
        src.set_property("port", self.port);
        src.set_property("do-timestamp", true);

        let caps = gst::Caps::new_simple(
            "application/x-rtp-stream",
            &[
                ("media", &"audio"),
                ("clock-rate", &48000i32),
                ("encoding-name", &"VORBIS"),
            ],
        );

        filter.set_property("caps", &caps);

        pipeline
            .add_many(&[&src, &filter, &rtpstreamdepay, &rtpvorbisdepay, &decodebin])
            .unwrap();

        gst::Element::link_many(&[&src, &filter, &rtpstreamdepay, &rtpvorbisdepay, &decodebin])
            .unwrap();

        let pipeline_weak = pipeline.downgrade();

        decodebin.connect_pad_added(move |dbin, src_pad| {
            let pipeline = match pipeline_weak.upgrade() {
                Some(pipeline) => pipeline,
                None => return,
            };

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

            let insert_sink = |is_audio, is_video| -> Result<(), Error> {
                if is_audio {
                    let queue = gst::ElementFactory::make("queue").build().unwrap();
                    let convert = gst::ElementFactory::make("audioconvert").build().unwrap();
                    let resample = gst::ElementFactory::make("audioresample").build().unwrap();
                    let sink = gst::ElementFactory::make("autoaudiosink").build().unwrap();

                    let elements = &[&queue, &convert, &resample, &sink];
                    pipeline.add_many(elements)?;
                    gst::Element::link_many(elements)?;

                    for e in elements {
                        e.sync_state_with_parent()?;
                    }

                    let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                    src_pad.link(&sink_pad)?;
                } else if is_video {
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

                    let sink_pad = queue.static_pad("sink").expect("queue has no sinkpad");
                    src_pad.link(&sink_pad)?;
                }

                Ok(())
            };

            if let Err(err) = insert_sink(is_audio, is_video) {
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
    }
}
