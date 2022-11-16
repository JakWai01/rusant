use gstreamer::prelude::*;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, MediaFile, Video};
use std::path::Path;

const gst_src: &str = "v4l2src device=";
const gst_src_format: &str = "video/x-raw,format=RGB";
const gst_videosink: &str = "ximagesink";
const sep: &str = " ! ";

// gst-launch-1.0 v4l2src name=cam_src ! decodebin ! videoconvert ! videoscale ! video/x-raw,format=RGB ! queue ! videoconvert ! ximagesink name=img_origin
fn main() {
    // Check if camera is plugged in
    if !Path::new("/dev/video0").exists() {
        panic!("No webcam detected: /dev/video0 cannot be found.")
    }

    // Gstreamer webcam
    gstreamer::init().unwrap();

    let src: String = gst_src.to_string() + "/dev/video0";
    let src_format: String =
        gst_src_format.to_string(); // + ",width=1920,height=1080,framerate=60";
    let videosink = gst_videosink;
    let video_pipeline = src + sep + &src_format + sep + videosink;

    println!("{}", video_pipeline);

    let command_pipeline = "v4l2src name=cam_src ! decodebin ! videoconvert ! videoscale ! video/x-raw,format=RGB ! queue ! videoconvert ! ximagesink name=img_origin";
    let pipeline = gstreamer::parse_launch(command_pipeline).unwrap();
    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");

    // GTK
    let app = Application::builder().application_id("rusant").build();
    app.connect_activate(build_ui);
    app.run();

    // Gstreamer
    gstreamer::init().unwrap();

    let uri =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gstreamer::parse_launch(&format!("playbin uri={}", uri)).unwrap();

    pipeline
        .set_state(gstreamer::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                break;
            }
            _ => (),
        }
    }

    pipeline
        .set_state(gstreamer::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}

fn build_ui(app: &Application) {
    let file = MediaFile::for_filename("video.mp4");

    let vid = Video::for_media_stream(Some(&file));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rusant")
        .child(&vid)
        .build();

    window.present();
}
