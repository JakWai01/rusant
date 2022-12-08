mod sender;
mod receiver;

use sender::Sender;
use receiver::Receiver;

use gtk::glib;
use config::Config;
use std::path::Path;
use std::collections::HashMap;
use std::env;

fn main() {
    println!("Hello World!");
    
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
 
    // let app = gtk::Application::new(None, gio::ApplicationFlags::FLAGS_NONE);

    match args.len() {
        // no arguments passed
        1 => {
            panic!("Please specify if you want to run the application as a 'sender' or a 'receiver'");
        },
        2 => {
            match args[1].as_str() {
                "sender" => {
                    let sender = sender::SenderPipeline{};
                    sender.send();
                },
                "receiver" => {
                    let receiver = receiver::ReceiverPipeline{};
                    receiver.receive();
                },
                _ => {
                    panic!("Invalid argument!")
                }
            }

        },
        _ => {
            println!("Too many arguments passed!")
        }
    }
    // app.connect_activate(build_ui);
    // app.run();

    unsafe {
        gst::deinit();
    }
}