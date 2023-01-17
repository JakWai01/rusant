use gst::prelude::*;
use log::info;

/// Sender part of the gstreamer pipeline
pub struct VideoSenderPipeline<'a> {
    host: &'a str,
    port: i32,
    pipeline: gst::Pipeline,
}

pub struct AudioSenderPipeline<'a> {
    host: &'a str,
    port: i32,
    pipeline: gst::Pipeline,
}

impl<'a> VideoSenderPipeline<'a> {
    /// Initialize a new VideoSenderPipeline
    pub fn new(host: &'a str, port: i32) -> Self {
        VideoSenderPipeline { host, port, pipeline: gst::Pipeline::new(Some("VideoSender")) }
    }
    
    /// Start sender pipeline
    pub fn start(&self) {
        // let pipeline = self.build();

        // Start pipeline
        self.pipeline.set_state(gst::State::Playing).unwrap();
    }

    /// Stop sender pipeline
    pub fn stop(&self) {
        self.pipeline.set_state(gst::State::Null).unwrap();
    }

    /// Build the pipeline
    pub fn build(&self) {
        info!("Initializing video sender pipeline");

        // Initialize Gstreamer pipeline
        // let pipeline = self.pipeline;

        // Initialize pads
        let v4l2src = gst::ElementFactory::make("v4l2src").build().unwrap();
        let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        let jpegenc = gst::ElementFactory::make("jpegenc").build().unwrap();
        let rtpjpegpay = gst::ElementFactory::make("rtpjpegpay").build().unwrap();
        let rtpstreampay = gst::ElementFactory::make("rtpstreampay").build().unwrap();
        let udpsink = gst::ElementFactory::make("tcpserversink").build().unwrap();
        // let udpsink = gst::ElementFactory::make("tcpclientsink").build().unwrap();
        
        // Initialize caps
        let caps = gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        filter.set_property("caps", &caps);

        udpsink.set_property("host", self.host);
        udpsink.set_property("port", self.port);

        // Add pads
        self.pipeline
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

        // self.pipeline = pipeline;
    }
}

impl<'a> AudioSenderPipeline<'a> {
    /// Initialize a new AudioSenderPipeline
    pub fn new(host: &'a str, port: i32) -> Self {
        AudioSenderPipeline { host, port, pipeline: gst::Pipeline::new(Some("AudioSender")) }
    }
    
    /// Start sender pipeline
    pub fn start(&self) {
        // let pipeline = self.build();

        // Start pipeline
        self.pipeline.set_state(gst::State::Playing).unwrap();
    }

    pub fn stop(&self) {
        self.pipeline.set_state(gst::State::Null).unwrap();
    }

    /// Build the pipeline
    pub fn build(&self) {
        info!("Initializing audio sender pipeline");

        // Initialize Gstreamer pipeline
        // let pipeline = gst::Pipeline::new(Some("AudioSender"));

        // Initialize pads
        let src = gst::ElementFactory::make("alsasrc").build().unwrap();
        let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        let vorbisenc = gst::ElementFactory::make("vorbisenc").build().unwrap();
        let rtpvorbispay = gst::ElementFactory::make("rtpvorbispay").build().unwrap();
        let rtpstreampay = gst::ElementFactory::make("rtpstreampay").build().unwrap();
        let sink = gst::ElementFactory::make("tcpserversink").build().unwrap();
        // let sink = gst::ElementFactory::make("tcpclientsink").build().unwrap();

        // Initialize caps
        let caps = gst::Caps::new_simple("audio/x-raw", &[("rate", &48000i32)]);

        filter.set_property("caps", &caps);

        rtpvorbispay.set_property("config-interval", 1u32);

        sink.set_property("host", self.host);
        sink.set_property("port", self.port);

        self.pipeline
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

        // self.pipeline = pipeline
    }
}
