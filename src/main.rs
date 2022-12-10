mod receiver;
mod sender;
mod ports;

use receiver::Receiver;
use sender::Sender;

use config::Config;
use gtk::glib;
use std::collections::HashMap;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse config
    let config = Config::builder()
        .add_source(config::File::with_name("Config"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let name = &config.get("name").unwrap();
    let app_id = &config.get("app_id").unwrap();

    // Check if video device exists
    if !Path::new("/dev/video0").exists() {
        panic!("No webcam detected: /dev/video0 cannot be found.")
    }

    // Initialize GTK
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    // Initialize Gstreamer
    gst::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Apply configuration
    glib::set_application_name(name);
    gtk::Window::set_default_icon_name(app_id);

    match args.len() {
        // no arguments passed
        1 => {
            let receiver = receiver::ReceiverPipeline {};
            receiver.receive();
        }
        2 => {
            match args[1].as_str() {
                "sender" => {
                    let sender = sender::SenderPipeline {};
                    sender.send();
                }
                "receiver" => {
                    /*
                    You can't provide args when working with gdk as it suspects
                    that this will be a filename and this would require additional configuration
                    */
                }
                _ => {
                    panic!("Invalid argument!")
                }
            }
        }
        _ => {
            println!("Too many arguments passed!")
        }
    }

    unsafe {
        gst::deinit();
    }
}
