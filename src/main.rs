mod receiver;
mod rusant_call_pane;
mod rusant_contact_dialog;
mod rusant_contact_item;
mod rusant_contact_list;
mod rusant_greeter;
mod rusant_main_window;
mod sender;

use gtk::traits::ButtonExt;
use libadwaita::prelude::MessageDialogExtManual;
use saltpanelo_sys::saltpanelo::{SaltpaneloOnRequestCallResponse, SaltpaneloAdapterLink};

use log::{info, debug};
use rusant_main_window::MainWindow;

use config::Config;
use glib::clone;
use gtk::{
    gdk::Display, glib, prelude::ActionMapExt, prelude::GtkApplicationExt, prelude::GtkWindowExt,
    CssProvider, StyleContext,
};
use gtk_macros::{action, spawn};
use std::ffi::{c_void, CString};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{self};
use std::path::Path;
use std::ptr::null_mut;
use std::{collections::HashMap, thread};

use gtk::gio::resources_register_include;

use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, HeaderBar, WindowTitle,
};

fn main() {
    // Initialize logger
    pretty_env_logger::init();

    // Initialize lock
    unsafe { LOCK = Some(Arc::new(Mutex::new(0))) }

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

    // Initialize GStreamer
    gst::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Initialize variables
    glib::set_application_name(name);

    gtk::Window::set_default_icon_name(app_id);

    gtk::init().unwrap();

    // Load gst-plugin-gtk4 GStreamer plugin
    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    // Load resources
    resources_register_include!("gtk-rusant.gresource").expect("Failed to register resources.");

    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();

    provider.load_from_resource("/com/jakobwaibel/Rusant/style.css");

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Initialize application
    let app = Application::builder()
        .application_id(app_id)
        .resource_base_path("/com/jakobwaibel/Rusant")
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .build();

    // Run application
    app.connect_activate(build_ui);

    let actions = gio::SimpleActionGroup::new();

    app.set_action_group(Some(&actions));

    setup_accels(&app);
    {
        action! {
            actions,
            "about",
            clone!(@weak app as app => move |_, _| {
            show_about(&app);
            })
        };

        action! {
            actions,
            "show-preferences",
            clone!(@weak app as app => move |_, _| {
                show_preferences(&app);
            })
        }
    }

    info!("Starting application");

    std::process::exit(app.run());
}

unsafe extern "C" fn open_url(
    url: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_char {
    open::that(std::ffi::CStr::from_ptr(url).to_str().unwrap()).unwrap();
    CString::new("").unwrap().into_raw()
}

pub static mut SRC_EMAIL: Option<String> = None;

// Possible CHANNEL_IDs are VIDEO_SENDER, VIDEO_RECEIVER, AUDIO_SENDER, AUDIO_RECEIVER, ONLY_AUDIO_SENDER and ONLY_AUDIO_RECEIVER
pub static mut CHANNEL_ID: Option<String> = None;

pub static mut RADDR: Option<String> = None;
pub static mut RPORT: Option<i32> = None;

// Set this to false if call was ended
pub static mut DIALOGUED: Option<bool> = None;

pub static mut REQUESTED_VIDEO_SENDER: bool = false;
pub static mut REQUESTED_VIDEO_RECEIVER: bool = false;
pub static mut REQUESTED_AUDIO_SENDER: bool = false;
pub static mut REQUESTED_AUDIO_RECEIVER: bool = false;
pub static mut REQUESTED_ONLY_AUDIO_SENDER: bool = false;
pub static mut REQUESTED_ONLY_AUDIO_RECEIVER: bool = false;

pub static mut LOCK: Option<Arc<Mutex<i32>>> = None;

pub static mut ACCEPTED: bool = false;

// if the first one was declined, decline all others 
unsafe extern "C" fn on_request_call(
    src_id: *mut ::std::os::raw::c_char,
    src_email: *mut ::std::os::raw::c_char,
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> SaltpaneloOnRequestCallResponse {
    let _lock = LOCK.as_ref().unwrap().lock().unwrap();
    info!("Call was requested");
    println!("Call was requested");
    match DIALOGUED.as_mut() {
        Some(_) => {},
        None => {
            DIALOGUED = Some(false);
        }
    }

    if !DIALOGUED.as_ref().unwrap() {
        let (sender, receiver) = mpsc::channel();

        glib::idle_add(move || {
            
            let sender = sender.clone();
            spawn!(async move {
                let accept = show_ring_dialog().await;
                sender.send(accept).expect("Could not send");
            });

            glib::Continue(false)
        });

        let accept = receiver.recv().unwrap();
        
        DIALOGUED = Some(true);

        if accept == 1 {
            ACCEPTED = true;
            SRC_EMAIL = Some(String::from(std::ffi::CStr::from_ptr(src_email).to_str().unwrap()));
            CHANNEL_ID = Some(String::from(std::ffi::CStr::from_ptr(channel_id).to_str().unwrap()));
        }

        if accept == 0 {
            ACCEPTED = false;
            REQUESTED_AUDIO_RECEIVER = false;
            REQUESTED_AUDIO_SENDER = false;
            REQUESTED_ONLY_AUDIO_RECEIVER = false;
            REQUESTED_ONLY_AUDIO_SENDER = false;
            REQUESTED_VIDEO_RECEIVER = false;
            REQUESTED_VIDEO_SENDER = false;

            DIALOGUED = Some(false);
        }

        SaltpaneloOnRequestCallResponse {
            Accept: accept,
            Err: CString::new("").unwrap().into_raw(),
        }
    } else {
        if ACCEPTED == true {
            SaltpaneloOnRequestCallResponse {
                Accept: 1,
                Err: CString::new("").unwrap().into_raw(),
            }
        } else {
            SaltpaneloOnRequestCallResponse {
                Accept: 0,
                Err: CString::new("").unwrap().into_raw(),
            }
        }
    }
}

pub async fn show_ring_dialog() -> i8 {
        info!("Showing ring dialog");

        let builder = gtk::Builder::from_resource("/com/jakobwaibel/Rusant/rusant-ring-dialog.ui");

        let dialog = builder
            .object::<libadwaita::MessageDialog>("dialog")
            .unwrap();

        unsafe {
            dialog.set_transient_for(Some(WINDOW.as_ref().unwrap()));
        };
        
        if dialog.run_future().await == "accept" {
            debug!("Accepting call");
            1
        } else {
            debug!("Denying the call");
            0
        }
    }

unsafe extern "C" fn on_call_disconnected(
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_char {
    let c_str = std::ffi::CStr::from_ptr(route_id);
    
    info!("Call with route ID {} was ended", c_str.to_str().unwrap());

    glib::idle_add(move || {
        WINDOW.as_ref().unwrap().call_pane().call_box().set_visible(false);
        WINDOW.as_ref().unwrap().call_pane().placeholder().set_visible(true);
        WINDOW.as_ref().unwrap().call_pane().action_bar().set_visible(false);

        WINDOW.as_ref().unwrap().call_pane().camera_video().add_css_class("suggested-action");
        WINDOW.as_ref().unwrap().call_pane().audio_input_microphone().add_css_class("suggested-action");

        while let Some(child) = WINDOW.as_ref().unwrap().call_pane().grid().child_at_index(0) {
            WINDOW.as_ref().unwrap().call_pane().grid().remove(&child);
        }

        // Stop pipelines
        if let Some(pipeline) = VIDEO_RECEIVER.as_ref() {
            pipeline.stop();
        }

        if let Some(pipeline) = VIDEO_SENDER.as_ref() {
            pipeline.stop()
        }

        if let Some(pipeline) = AUDIO_RECEIVER.as_ref() {
            pipeline.stop()
        }
        
        if let Some(pipeline) = AUDIO_SENDER.as_ref() {
            pipeline.stop()
        }

        REQUESTED_AUDIO_RECEIVER = false;
        REQUESTED_AUDIO_SENDER = false;
        REQUESTED_VIDEO_RECEIVER = false;
        REQUESTED_VIDEO_SENDER = false;
        REQUESTED_ONLY_AUDIO_RECEIVER = false;
        REQUESTED_ONLY_AUDIO_SENDER = false;

        VIDEO_RECEIVER = None;
        VIDEO_SENDER = None;
        AUDIO_RECEIVER = None;
        AUDIO_SENDER = None;

        VIDEO_RECEIVER_ROUTE_ID = None;
        VIDEO_SENDER_ROUTE_ID = None;
        AUDIO_SENDER_ROUTE_ID = None;
        AUDIO_RECEIVER_ROUTE_ID = None;

        DIALOGUED = Some(false);

        glib::Continue(false)
    });

    CString::new("").unwrap().into_raw()
}

pub static mut VIDEO_SENDER_ROUTE_ID: Option<String> = None;
pub static mut VIDEO_RECEIVER_ROUTE_ID: Option<String> = None;
pub static mut AUDIO_SENDER_ROUTE_ID: Option<String> = None;
pub static mut AUDIO_RECEIVER_ROUTE_ID: Option<String> = None;

pub static mut VIDEO_RECEIVER: Option<receiver::VideoReceiverPipeline> = None;
pub static mut VIDEO_SENDER: Option<sender::VideoSenderPipeline> = None;
pub static mut AUDIO_SENDER: Option<sender::AudioSenderPipeline> = None;
pub static mut AUDIO_RECEIVER: Option<receiver::AudioReceiverPipeline> = None;

unsafe extern "C" fn on_handle_call(
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    raddr: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_char {
    let route_id_c_str = std::ffi::CStr::from_ptr(route_id).to_str().unwrap();
    let raddr_c_str = std::ffi::CStr::from_ptr(raddr);

    info!(
        "Call with route ID {:?} and remote address {:?} started",
        route_id_c_str, raddr_c_str
    );

    // Split original raddr into address and port
    if let Some((address, port)) = std::ffi::CStr::from_ptr(raddr).to_str().unwrap().split_once(':') {
        RADDR = Some(String::from(address));
        RPORT = Some(port.parse().unwrap());
    }

    let address = RADDR.clone().unwrap();
    let port = RPORT.unwrap();
    let channel = std::ffi::CStr::from_ptr(channel_id).to_str().unwrap();

    match channel {
        "VIDEO_SENDER" => {
            VIDEO_SENDER_ROUTE_ID = Some(String::from(route_id_c_str));
        },
        "VIDEO_RECEIVER" => {
            VIDEO_RECEIVER_ROUTE_ID = Some(String::from(route_id_c_str));
        },
        "AUDIO_SENDER" | "ONLY_AUDIO_SENDER" => {
            AUDIO_SENDER_ROUTE_ID = Some(String::from(route_id_c_str));
        },
        "AUDIO_RECEIVER" | "ONLY_AUDIO_RECEIVER" => {
            AUDIO_RECEIVER_ROUTE_ID = Some(String::from(route_id_c_str));
        },
        &_ => unimplemented!()
    }

    info!("Partner's address is {} and their port is {}", address, port);

    glib::idle_add(move || {
        if channel == "VIDEO_SENDER" && REQUESTED_VIDEO_SENDER {
            VIDEO_RECEIVER = Some(receiver::VideoReceiverPipeline::new(address.clone(), port));
            let paintable = VIDEO_RECEIVER.as_ref().unwrap().build();
            VIDEO_RECEIVER.as_ref().unwrap().start();

            let picture = gtk::Picture::new();
            picture.set_paintable(Some(&paintable));

            WINDOW.as_ref().unwrap().call_pane().grid().insert(&picture, 0);
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        } else if channel == "VIDEO_SENDER" {
            VIDEO_SENDER = Some(sender::VideoSenderPipeline::new(address.clone(), port));
            let paintable = VIDEO_SENDER.as_ref().unwrap().build();
            VIDEO_SENDER.as_ref().unwrap().start();
            
            let picture = gtk::Picture::new();
            picture.set_paintable(Some(&paintable));

            WINDOW.as_ref().unwrap().call_pane().grid().insert(&picture, 0);
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        }

        if channel == "VIDEO_RECEIVER" && REQUESTED_VIDEO_RECEIVER {
            VIDEO_SENDER = Some(sender::VideoSenderPipeline::new(address.clone(), port));
            let paintable = VIDEO_SENDER.as_ref().unwrap().build();
            VIDEO_SENDER.as_ref().unwrap().start();
            
            let picture = gtk::Picture::new();
            picture.set_paintable(Some(&paintable));

            WINDOW.as_ref().unwrap().call_pane().grid().insert(&picture, 0);
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        } else if channel == "VIDEO_RECEIVER" {
            VIDEO_RECEIVER = Some(receiver::VideoReceiverPipeline::new(address.clone(), port));
            let paintable = VIDEO_RECEIVER.as_ref().unwrap().build();
            VIDEO_RECEIVER.as_ref().unwrap().start();

            let picture = gtk::Picture::new();
            picture.set_paintable(Some(&paintable));

            WINDOW.as_ref().unwrap().call_pane().grid().insert(&picture, 0);
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        }

        if channel == "AUDIO_SENDER" && REQUESTED_AUDIO_SENDER {
            AUDIO_RECEIVER = Some(receiver::AudioReceiverPipeline::new(address.clone(), port));
            AUDIO_RECEIVER.as_ref().unwrap().build();
            AUDIO_RECEIVER.as_ref().unwrap().start();
            
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        } else if channel == "AUDIO_SENDER" {
            AUDIO_SENDER = Some(sender::AudioSenderPipeline::new(address.clone(), port));
            AUDIO_SENDER.as_ref().unwrap().build();
            AUDIO_SENDER.as_ref().unwrap().start();
            
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        } 

        if channel == "AUDIO_RECEIVER" && REQUESTED_AUDIO_RECEIVER {
            AUDIO_SENDER = Some(sender::AudioSenderPipeline::new(address.clone(), port));
            AUDIO_SENDER.as_ref().unwrap().build();
            AUDIO_SENDER.as_ref().unwrap().start();
            
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        } else if channel == "AUDIO_RECEIVER" {
            AUDIO_RECEIVER = Some(receiver::AudioReceiverPipeline::new(address.clone(), port));
            AUDIO_RECEIVER.as_ref().unwrap().build();
            AUDIO_RECEIVER.as_ref().unwrap().start();
            
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(true);
        }

        if channel == "ONLY_AUDIO_SENDER" && REQUESTED_ONLY_AUDIO_SENDER {
            AUDIO_RECEIVER = Some(receiver::AudioReceiverPipeline::new(address.clone(), port));
            AUDIO_RECEIVER.as_ref().unwrap().build();
            AUDIO_RECEIVER.as_ref().unwrap().start();

            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(false);
        } else if channel == "ONLY_AUDIO_SENDER" {
            AUDIO_SENDER = Some(sender::AudioSenderPipeline::new(address.clone(), port));
            AUDIO_SENDER.as_ref().unwrap().build();
            AUDIO_SENDER.as_ref().unwrap().start();
            
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(false);
        }

        if channel == "ONLY_AUDIO_RECEIVER" && REQUESTED_ONLY_AUDIO_RECEIVER {
            AUDIO_SENDER = Some(sender::AudioSenderPipeline::new(address.clone(), port));
            AUDIO_SENDER.as_ref().unwrap().build();
            AUDIO_SENDER.as_ref().unwrap().start();
            
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(false);
        } else if channel == "ONLY_AUDIO_RECEIVER" {
            AUDIO_RECEIVER = Some(receiver::AudioReceiverPipeline::new(address.clone(), port));
            AUDIO_RECEIVER.as_ref().unwrap().build();
            AUDIO_RECEIVER.as_ref().unwrap().start();
            
            WINDOW.as_ref().unwrap().call_pane().camera_video().set_visible(false);
        }

        // Open call pane
        WINDOW.as_ref().unwrap().call_pane().call_box().set_visible(true);
        WINDOW.as_ref().unwrap().call_pane().placeholder().set_visible(false);
        WINDOW.as_ref().unwrap().call_pane().action_bar().set_visible(true);

        glib::Continue(false)
    });

    CString::new("").unwrap().into_raw()
}

pub static mut ADAPTER: Option<usize> = None;
pub static mut WINDOW: Option<MainWindow> = None;

/// Build the user interface
fn build_ui(app: &Application) {
    let content = libadwaita::gtk::Box::new(Orientation::Vertical, 0);

    content.append(
        &HeaderBar::builder()
            .title_widget(&WindowTitle::new("Rusant", ""))
            .build(),
    );

    let mut window: Option<MainWindow> = None;

    unsafe {
        let user_data_ptr = &mut window as *mut Option<MainWindow> as *mut c_void;

        let ptr = saltpanelo_sys::saltpanelo::SaltpaneloNewAdapter(
            Some(on_request_call),
            null_mut(),
            Some(on_call_disconnected),
            null_mut(),
            Some(on_handle_call),
            null_mut(),
            Some(open_url),
            user_data_ptr,
            CString::new("ws://localhost:1338").unwrap().into_raw(),
            CString::new("127.0.0.1").unwrap().into_raw(),
            0,
            10000,
            CString::new("https://pojntfx.eu.auth0.com/")
                .unwrap()
                .into_raw(),
            CString::new("An94hvwzqxMmFcL8iEpTVrd88zFdhVdl")
                .unwrap()
                .into_raw(),
            CString::new("http://localhost:11337").unwrap().into_raw(),
        );

        ADAPTER = Some(ptr as usize);

        WINDOW = Some(MainWindow::new(app));

        let win = WINDOW.as_ref().unwrap();
        win.greeter()
            .login_button()
            .connect_clicked(clone!(@weak win => move |_| {
                    let res = saltpanelo_sys::saltpanelo::SaltpaneloAdapterLogin(ptr);

                    let c_str = std::ffi::CStr::from_ptr(res);

                    let n_ptr = ptr as usize;
                    
                    thread::spawn(move || {
                        let rv = SaltpaneloAdapterLink(n_ptr as *mut c_void);

                        if !std::ffi::CStr::from_ptr(rv).to_str().unwrap().eq("") {
                            info!(
                                "Error in SaltpaneloAdapterLink: {}",
                                std::ffi::CStr::from_ptr(rv).to_str().unwrap()
                            );
                        }
                    });

                    win.switch_to_leaflet();
                }));
    }

    info!("Building UI");
    unsafe {
        WINDOW.as_ref().unwrap().show();
    }
}

/// Show the about page
fn show_about(app: &Application) {
    let window = app.active_window().unwrap();

    let dialog = libadwaita::AboutWindow::builder()
        .transient_for(&window)
        .application_name("Rusant")
        .developer_name("Jakob Waibel")
        .version("0.0.1")
        .developers(vec!["Jakob Waibel".into(), "Felicitas Pojtinger".into()])
        .copyright("© 2022 Jakob Waibel")
        .website("https://github.com/JakWai01/rusant")
        .issue_url("https://github.com/JakWai01/rusant/issues/new")
        .license_type(gtk::License::Agpl30)
        .build();

    info!("Showing about page");

    dialog.present();
}

/// Show the preferences page
fn show_preferences(app: &Application) {
    let window = app.active_window().unwrap();

    let dialog = libadwaita::PreferencesWindow::builder()
        .transient_for(&window)
        .build();

    info!("Showing preferences");

    dialog.present();
}

/// Setup keyboard shortcuts
fn setup_accels(app: &Application) {
    app.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
}
