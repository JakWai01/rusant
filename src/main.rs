use gst::element_error;
use gst::prelude::*;

use byte_slice_cast::*;

use std::i16;

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

    appsink.set_callbacks(
        gst_audio::AppSinkCallbacks::builder()
            .new_sample(|appsink| {
                let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
                let buffer = sample.buffer().ok_or_else(|| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to get buffer from appsink")
                    );

                    gst::FlowError::Error
                })?;

                let map = buffer.map_readable().map_err(|_| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to map buffer readable")
                    );

                    gst::FlowError::Error
                })?;

                let samples = map.as_slice_of::<i16>().map_err(|_| {
                    element_error!(
                        appsink,
                        gst::ResourceError::Failed,
                        ("Failed to interprete buffer as S16 PCM")
                    );

                    gst::FlowError::Error
                })?;

                println!("{:?}", samples);

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
            MessageView::Error(_err) => {
                pipeline.set_state(gst::State::Null).unwrap();
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();

    Ok(())
}

fn main() {
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}