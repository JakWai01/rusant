mod ports;
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
use rusant_sys::add;
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
use std::collections::HashSet;
use std::ffi::{c_void, CString};
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
    // println!("We did it! We did it!");

    // let win = &*(userdata as *mut MainWindow);
    // println!("Pointer before idle: {:?}", userdata);
    // let n_ptr = userdata as usize;

    // let app = gtk::Application::new(None, Default::default());
    // app.connect_activate(move |app| {
    //     let window = gtk::ApplicationWindow::new(app);
    //     window.set_default_size(800, 500);
    //     window.set_title(Some("Rusant"));

    //     let context = WebContext::default().unwrap();
    //     let webview = WebView::with_context(&context);
    //     webview.load_uri(std::ffi::CStr::from_ptr(url).to_str().unwrap());
    //     window.set_child(Some(&webview));

    //     let settings = WebViewExt::settings(&webview).unwrap();
    //     settings.set_enable_developer_extras(true);

    //     window.show();
    // });
    open::that(std::ffi::CStr::from_ptr(url).to_str().unwrap()).unwrap();

    // let win = &*(userdata as *mut MainWindow);
    // println!("Is visible: {}", win.is_visible());
    // glib::idle_add(move || {
    //     println!("We are in idle");
    //     println!("Pointer in idle: {:?}", n_ptr as *mut c_void);
    //     let win = &*(n_ptr as *mut c_void as *mut MainWindow);
    //     // win.switch_to_leaflet();
    //     println!("{}", win.is_visible());
    //     Continue(false)
    // });

    // win.switch_to_leaflet();
    // app.connect_shutdown(clone!(@weak win => move |_| {
    //     info!("Window was closed. Successfully authenticated!");

    //     /*
    //      * This is the success case if the authentication worked
    //      * Later, this handler should close the application window
    //      */
    //     win.switch_to_leaflet()
    // }));
    // app.run();

    // win.switch_to_leaflet();

    // println!("The desired name is: {:?}", );
    // What should we return here?
    CString::new("").unwrap().into_raw()
}

pub static mut ROUTE_ID: Option<String> = None;
pub static mut SRC_EMAIL: Option<String> = None;

// Possible CHANNEL_IDs are VIDEO_SENDER, VIDEO_RECEIVER, AUDIO_SENDER, AUDIO_RECEIVER
pub static mut CHANNEL_ID: Option<String> = None;

pub static mut RADDR: Option<String> = None;
pub static mut RPORT: Option<i32> = None;

// Set this to false if call was ended
pub static mut DIALOGUED: Option<bool> = None;

// Experimental 
pub static mut REQUESTED_VIDEO_SENDER: bool = false;
pub static mut REQUESTED_VIDEO_RECEIVER: bool = false;
pub static mut REQUESTED_AUDIO_SENDER: bool = false;
pub static mut REQUESTED_AUDIO_RECEIVER: bool = false;

unsafe extern "C" fn on_request_call(
    src_id: *mut ::std::os::raw::c_char,
    src_email: *mut ::std::os::raw::c_char,
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> SaltpaneloOnRequestCallResponse {
    println!("Requested call");

    ROUTE_ID = Some(String::from(std::ffi::CStr::from_ptr(route_id).to_str().unwrap()));
    SRC_EMAIL = Some(String::from(std::ffi::CStr::from_ptr(src_email).to_str().unwrap()));
    CHANNEL_ID = Some(String::from(std::ffi::CStr::from_ptr(channel_id).to_str().unwrap()));

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
        
        println!("Accept is currently: {}", accept);
    
        DIALOGUED = Some(true);

        SaltpaneloOnRequestCallResponse {
            Accept: accept,
            Err: CString::new("").unwrap().into_raw(),
        }
    } else {
        SaltpaneloOnRequestCallResponse {
            Accept: 1,
            Err: CString::new("").unwrap().into_raw(),
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
            println!("Accepting call");
            1
        } else {
            debug!("Denying the call");
            println!("Denying call");
            0
        }
    }

unsafe extern "C" fn on_call_disconnected(
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_char {
    let c_str = std::ffi::CStr::from_ptr(route_id);
    println!("Call with route ID {} was ended", c_str.to_str().unwrap());

    glib::idle_add(move || {
        WINDOW.as_ref().unwrap().call_pane().call_box().set_visible(false);
        WINDOW.as_ref().unwrap().call_pane().placeholder().set_visible(true);
        WINDOW.as_ref().unwrap().call_pane().action_bar().set_visible(false);

        while let Some(child) = WINDOW.as_ref().unwrap().call_pane().grid().child_at_index(0) {
            WINDOW.as_ref().unwrap().call_pane().grid().remove(&child);
        }

        glib::Continue(false)
    });

    CString::new("").unwrap().into_raw()
}

unsafe extern "C" fn on_handle_call(
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    raddr: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_char {
    let route_id_c_str = std::ffi::CStr::from_ptr(route_id);
    let raddr_c_str = std::ffi::CStr::from_ptr(raddr);

    println!(
        "Call with route ID {:?} and remote address {:?} started",
        route_id_c_str, raddr_c_str
    );

    ROUTE_ID = Some(String::from(std::ffi::CStr::from_ptr(route_id).to_str().unwrap()));

    // Split original raddr into address and port
    if let Some((address, port)) = std::ffi::CStr::from_ptr(raddr).to_str().unwrap().split_once(':') {
        RADDR = Some(String::from(address));
        RPORT = Some(port.parse().unwrap());
    }

    let address = RADDR.clone().unwrap();
    let port = RPORT.unwrap();
    let channel = std::ffi::CStr::from_ptr(channel_id).to_str().unwrap();

    println!("Partner's address is {} and their port is {}", address, port);

    glib::idle_add(move || {
        if channel == "VIDEO_SENDER" && REQUESTED_VIDEO_SENDER {
            println!("Receiving video");
            let receiver = receiver::VideoReceiverPipeline::new(&address, port);
            let paintable = receiver.build();
            receiver.start();

            let picture = gtk::Picture::new();
            picture.set_paintable(Some(&paintable));

            WINDOW.as_ref().unwrap().call_pane().grid().insert(&picture, 0);
        } else if channel == "VIDEO_SENDER" {
            println!("Sending video");
            let sender = sender::VideoSenderPipeline::new(&address, port);
            sender.build();
            sender.start();
        }

        if channel == "VIDEO_RECEIVER" && REQUESTED_VIDEO_RECEIVER {
            println!("Sending video");
            let sender = sender::VideoSenderPipeline::new(&address, port);
            sender.build();
            sender.start();
        } else if channel == "VIDEO_RECEIVER" {
            println!("Receiving video");
            let receiver = receiver::VideoReceiverPipeline::new(&address, port);
            let paintable = receiver.build();
            receiver.start();

            let picture = gtk::Picture::new();
            picture.set_paintable(Some(&paintable));

            WINDOW.as_ref().unwrap().call_pane().grid().insert(&picture, 0);
        }

        if channel == "AUDIO_SENDER" && REQUESTED_AUDIO_SENDER {

        } else if channel == "AUDIO_SENDER" {

        }

        if channel == "AUDIO_RECEIVER" && REQUESTED_AUDIO_RECEIVER {

        } else if channel == "AUDIO_RECEIVER" {

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
    // let mut window = MainWindow::new(app);

    unsafe {
        let user_data_ptr = &mut window as *mut Option<MainWindow> as *mut c_void;
        println!("{:?}", user_data_ptr);

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
                    println!("Pointer: {:#?}", ptr);
                    let res = saltpanelo_sys::saltpanelo::SaltpaneloAdapterLogin(ptr);

                    let c_str = std::ffi::CStr::from_ptr(res);

                    println!("{:?}", c_str.to_str().unwrap());
                    
                    let n_ptr = ptr as usize;
                    
                    thread::spawn(move || {
                        println!("{:?}", n_ptr as *mut c_void);
                        let rv = SaltpaneloAdapterLink(n_ptr as *mut c_void);

                        if !std::ffi::CStr::from_ptr(rv).to_str().unwrap().eq("") {
                            println!(
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
