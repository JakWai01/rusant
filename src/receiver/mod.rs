use gst::prelude::*;
use gtk::gdk;
use log::info;

/// Receiver part of the gstreamer pipeline
pub struct ReceiverPipeline<'a> {
    address: &'a str,
    port: i32,
}

impl<'a> ReceiverPipeline<'a> {
    /// Initialize a new ReceiverPipeline
    pub fn new(address: &'a str, port: i32) -> Self {
        ReceiverPipeline { address, port }
    }

    /// Build the pipeline
    pub fn build(&self) -> (gst::Pipeline, gdk::Paintable) {
        info!("Initializing receiver pipeline");

        // Initialize Gstreamer pipeline
        let pipeline = gst::Pipeline::new(None);

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
        let caps = gst::Caps::new_simple(
            "application/x-rtp-stream",
            &[("encoding-name", &"JPEG")],
        );
        filter.set_property("caps", &caps);

        // Create paintable to paint the webcam picture in
        let paintable = sink.property::<gdk::Paintable>("paintable");

        // Add pads
        pipeline
            .add_many(&[&src, &filter, &rtpstreamdepay, &rtpjpegdepay, &jpegdec, &sink])
            .unwrap();

        // Link pads
        gst::Element::link_many(&[&src, &filter, &rtpstreamdepay, &rtpjpegdepay, &jpegdec, &sink]).unwrap();

        (pipeline, paintable)
    }
}
