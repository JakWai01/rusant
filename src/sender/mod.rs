use gst::prelude::*;
use log::info;

pub trait Sender {
    fn send(&self) {}
}

/// Sender part of the gstreamer pipeline
pub struct VideoSenderPipeline<'a> {
    host: &'a str,
    port: i32,
}

pub struct AudioSenderPipeline<'a> {
    host: &'a str,
    port: i32,
}

impl<'a> Sender for VideoSenderPipeline<'a> {
    /// Start sender pipeline
    fn send(&self) {
        let pipeline = self.build();

        // Start pipeline
        pipeline.set_state(gst::State::Playing).unwrap();
    }
}

impl<'a> Sender for AudioSenderPipeline<'a> {
    /// Start sender pipeline
    fn send(&self) {
        let pipeline = self.build();

        // Start pipeline
        pipeline.set_state(gst::State::Playing).unwrap();
    }
}

impl<'a> VideoSenderPipeline<'a> {
    /// Initialize a new VideoSenderPipeline
    pub fn new(host: &'a str, port: i32) -> Self {
        VideoSenderPipeline { host, port }
    }

    /// Build the pipeline
    pub fn build(&self) -> gst::Pipeline {
        info!("Initializing video sender pipeline");

        // Initialize Gstreamer pipeline
        let pipeline = gst::Pipeline::new(Some("VideoSender"));

        // Initialize pads
        let v4l2src = gst::ElementFactory::make("v4l2src").build().unwrap();
        let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        let jpegenc = gst::ElementFactory::make("jpegenc").build().unwrap();
        let rtpjpegpay = gst::ElementFactory::make("rtpjpegpay").build().unwrap();
        let rtpstreampay = gst::ElementFactory::make("rtpstreampay").build().unwrap();
        let udpsink = gst::ElementFactory::make("tcpserversink").build().unwrap();

        // Initialize caps
        let caps = gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        filter.set_property("caps", &caps);

        udpsink.set_property("host", self.host);
        udpsink.set_property("port", self.port);

        // Add pads
        pipeline
            .add_many(&[
                &v4l2src,
                &filter,
                &jpegenc,
                &rtpjpegpay,
                &rtpstreampay,
                &udpsink,
            ])
            .unwrap();

        // Link pads
        gst::Element::link_many(&[
            &v4l2src,
            &filter,
            &jpegenc,
            &rtpjpegpay,
            &rtpstreampay,
            &udpsink,
        ])
        .unwrap();

        pipeline
    }
}

impl<'a> AudioSenderPipeline<'a> {
    /// Initialize a new AudioSenderPipeline
    pub fn new(host: &'a str, port: i32) -> Self {
        AudioSenderPipeline { host, port }
    }

    /// Build the pipeline
    pub fn build(&self) -> gst::Pipeline {
        info!("Initializing audio sender pipeline");

        // Initialize Gstreamer pipeline
        let pipeline = gst::Pipeline::new(Some("AudioSender"));

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

        sink.set_property("host", self.host);
        sink.set_property("port", self.port);

        pipeline
            .add_many(&[
                &src,
                &filter,
                &vorbisenc,
                &rtpvorbispay,
                &rtpstreampay,
                &sink,
            ])
            .unwrap();

        gst::Element::link_many(&[
            &src,
            &filter,
            &vorbisenc,
            &rtpvorbispay,
            &rtpstreampay,
            &sink,
        ])
        .unwrap();

        pipeline
    }
}
