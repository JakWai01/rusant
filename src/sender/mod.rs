use gst::prelude::*;

pub trait Sender {
    fn send(&self) {}
}

pub struct SenderPipeline<'a> {
    host: &'a str,
    port: i32,
}

impl<'a> Sender for SenderPipeline<'a> {
    fn send(&self) {
        let pipeline = self.build();

        // Start pipeline
        pipeline.set_state(gst::State::Playing).unwrap();

        let bus = pipeline
            .bus()
            .expect("Pipeline without bus. Shouldn't happend!");

        // Listen on bus for events
        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(_err) => {
                    pipeline.set_state(gst::State::Null).unwrap();
                }
                _ => (),
            }
        }

        // Stop pipeline
        pipeline.set_state(gst::State::Null).unwrap();
    }
}

impl<'a> SenderPipeline<'a> {
    pub fn new(host: &'a str, port: i32) -> Self {
        SenderPipeline { host, port }
    }

    pub fn build(&self) -> gst::Pipeline {
        // Initialize Gstreamer pipeline
        let pipeline = gst::Pipeline::new(Some("Sender"));

        // Initialize pads
        let v4l2src = gst::ElementFactory::make("v4l2src").build().unwrap();
        let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        let jpegenc = gst::ElementFactory::make("jpegenc").build().unwrap();
        let rtpjpegpay = gst::ElementFactory::make("rtpjpegpay").build().unwrap();
        let udpsink = gst::ElementFactory::make("udpsink").build().unwrap();

        let caps = gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);
        filter.set_property("caps", &caps);

        // v4l2src.set_caps(Some(&video_caps));
        udpsink.set_property("host", self.host);
        udpsink.set_property("port", self.port);

        // Add pads
        pipeline
            .add_many(&[&v4l2src, &filter, &jpegenc, &rtpjpegpay, &udpsink])
            .unwrap();

        // Link pads
        gst::Element::link_many(&[&v4l2src, &filter, &jpegenc, &rtpjpegpay, &udpsink]).unwrap();

        pipeline
    }
}
