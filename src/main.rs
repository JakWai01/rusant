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
use libadwaita::ApplicationWindow;
use rusant_sys::add;
use saltpanelo_sys::saltpanelo::{SaltpaneloOnRequestCallResponse, SaltpaneloAdapterLink};
use saltpanelo_sys::tti;

use log::info;
use rusant_main_window::MainWindow;

use config::Config;
use glib::{clone, Continue, ObjectExt, ToValue, Value};
use gtk::{
    gdk::Display, glib, prelude::ActionMapExt, prelude::GtkApplicationExt, prelude::GtkWindowExt,
    CssProvider, StyleContext, Window,
};
use gtk_macros::action;
use std::ffi::{c_void, CString};
use webkit2gtk::traits::{WebViewExt, WebkitSettingsExt};
use webkit2gtk::{WebContext, WebView};
// use webkit2gtk::{WebContext, WebView, WebViewExt, SettingsExt, WebContextExt};
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
    // tti();

    // badd();

    // println!("Result of shared operation: {:?}", key());

    println!("Result of lib operation: {:?}", add(1, 2));

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

unsafe extern "C" fn on_request_call(
    src_id: *mut ::std::os::raw::c_char,
    src_email: *mut ::std::os::raw::c_char,
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> SaltpaneloOnRequestCallResponse {
    println!("Requested call");

    // What should we return?
    SaltpaneloOnRequestCallResponse {
        Accept: 1,
        Err: CString::new("").unwrap().into_raw(),
    }
}

unsafe extern "C" fn on_call_disconnected(
    route_id: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_char {
    let c_str = std::ffi::CStr::from_ptr(route_id);
    println!("Call with route ID {} was ended", c_str.to_str().unwrap());

    // What should we return?
    route_id
}

unsafe extern "C" fn on_handle_call(
    route_id: *mut ::std::os::raw::c_char,
    raddr: *mut ::std::os::raw::c_char,
    userdata: *mut ::std::os::raw::c_void,
) -> *mut ::std::os::raw::c_char {
    let route_id_c_str = std::ffi::CStr::from_ptr(route_id);
    let raddr_c_str = std::ffi::CStr::from_ptr(raddr);

    println!(
        "Call with route ID {:?} and remote address {:?} started",
        route_id_c_str, raddr_c_str
    );

    // What should we return?
    route_id
}

pub static mut ADAPTER: Option<usize> = None;

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

        window = Some(MainWindow::new(app));

        let win = window.as_ref().unwrap();
        win.greeter()
            .login_button()
            .connect_clicked(clone!(@weak win => move |_| {
                    // let app = gtk::Application::new(None, Default::default());
                    // app.connect_activate(move |app| {
                    //     let window = ApplicationWindow::new(app);
                    //     window.set_default_size(800, 500);
                    //     window.set_title(Some("Rusant"));

                    //     let context = WebContext::default().unwrap();
                    //     let webview = WebView::with_context(&context);
                    //     webview.load_uri("https://github.com/JakWai01/rusant");
                    //     window.set_child(Some(&webview));

                    //     let settings = WebViewExt::settings(&webview).unwrap();
                    //     settings.set_enable_developer_extras(true);

                    //     window.show();
                    // });

                    // app.connect_shutdown(move |_| {
                    //     info!("Window was closed. Successfully authenticated!");

                    //     /*
                    //      * This is the success case if the authentication worked
                    //      * Later, this handler should close the application window
                    //      */
                    //     window.switch_to_leaflet()
                    // });
                    // app.run();

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
                                "Error in SalpaneloAdapterLink: {}",
                                std::ffi::CStr::from_ptr(rv).to_str().unwrap()
                            );
                        }
                    });

                    win.switch_to_leaflet();
                }));

        // let win = &*(user_data_ptr as *mut c_void as *mut MainWindow);
        // win.show();

        // println!("{:#?}", ptr);

        // let res = saltpanelo_sys::saltpanelo::SaltpaneloAdapterLogin(ptr);

        // let c_str = std::ffi::CStr::from_ptr(res);

        // println!("{:?}", c_str.to_str().unwrap());
    }

    info!("Building UI");
    // println!("{:?}", window.unwrap().property::<*mut c_void>("ptr"));
    window.unwrap().show();
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
