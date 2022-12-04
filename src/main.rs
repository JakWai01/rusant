use gst::element_error;
use gst::prelude::*;

use byte_slice_cast::*;

use std::i16;
use std::i32;

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init().unwrap();

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("autovideosrc").build().unwrap();
    let convert = gst::ElementFactory::make("videoconvert").build().unwrap();
    let sink = gst::ElementFactory::make("appsink").build().unwrap();

    pipeline.add_many(&[&src, &convert, &sink]).unwrap();
    gst::Element::link_many(&[&src, &convert, &sink]).unwrap();

    let appsink = sink
        .dynamic_cast::<gst_audio::AppSink>()
        .expect("Sink element is expected to be an appsink!");

    // Tell the appsink what format we want. It will then be the audiotestsrc's job to
    // provide the format we request.
    // This can be set after linking the two objects, because format negotiation between
    // both elements will happen during pre-rolling of the pipeline.
    // appsink.set_caps(Some(
    //     &gst::Caps::builder("audio/x-raw")
    //         .field("format", gst_app::AUDIO_FORMAT_S16.to_str())
    //         .field("layout", "interleaved")
    //         .field("channels", 1i32)
    //         .field("rate", gst::IntRange::<i32>::new(1, i32::MAX))
    //         .build(),
    // ));

    // Getting data out of the appsink is done by setting callbacks on it.
    // The appsink will then call those handlers, as soon as data is available.
    appsink.set_callbacks(
        gst_audio::AppSinkCallbacks::builder()
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