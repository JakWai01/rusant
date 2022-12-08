use gst::prelude::*;

pub trait Receiver {
    fn receive(&self){}
}

pub struct ReceiverPipeline{}

impl Receiver for ReceiverPipeline {
    fn receive(&self){
        println!("Client: Hello!");

        // Initialize Gstreamer
        gst::init().unwrap();

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
        let sink = gst::ElementFactory::make("autovideosink").build().unwrap();

        let caps = gst::Caps::new_simple(
            "application/x-rtp",
            &[("encoding-name", &"JPEG"), ("payload", &26i32)],
        );
        filter.set_property("caps", &caps);

        // Add pads
        pipeline
            .add_many(&[&src, &filter, &rtpjpegdepay, &jpegdec, &sink])
            .unwrap();

        // Link pads
        gst::Element::link_many(&[&src, &filter, &rtpjpegdepay, &jpegdec, &sink]).unwrap();

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