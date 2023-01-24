use crate::{receiver, ADAPTER, WINDOW, VIDEO_SENDER_ROUTE_ID, VIDEO_RECEIVER_ROUTE_ID, AUDIO_SENDER_ROUTE_ID, AUDIO_RECEIVER_ROUTE_ID};

use super::CallPane;

use std::{thread, os::raw::c_void, ffi::CString};

use anyhow::Error;
use derive_more::{Display, Error};
use gio::subclass::prelude::ObjectSubclassExt;
use glib::{
    self, clone, object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    ObjectExt,
};
use libadwaita::{HeaderBar, StatusPage};

use gst::{
    element_error, element_warning, prelude::GstBinExtManual, prelude::*, traits::ElementExt,
};

use gtk::{
    gdk,
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    traits::{ButtonExt, WidgetExt},
    ActionBar, Box, Button, CompositeTemplate, FlowBox,
};
use log::info;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-call-pane.ui")]
pub struct CallPaneTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub back_button: TemplateChild<Button>,

    #[template_child]
    pub grid: TemplateChild<FlowBox>,

    #[template_child]
    pub placeholder: TemplateChild<StatusPage>,

    #[template_child]
    pub call_box: TemplateChild<Box>,

    #[template_child]
    pub action_bar: TemplateChild<ActionBar>,

    #[template_child]
    pub camera_video: TemplateChild<Button>,

    #[template_child]
    pub audio_input_microphone: TemplateChild<Button>,

    #[template_child]
    pub call_stop: TemplateChild<Button>,
}

#[object_subclass]
impl ObjectSubclass for CallPaneTemplate {
    const NAME: &'static str = "CallPane";

    type Type = CallPane;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CallPaneTemplate {
    /// Construct a new CallPane
    fn constructed(&self) {
        self.parent_constructed();

        // Handle click on camera_video button
        self.camera_video.connect_clicked(move |button| {
            info!("Button `camera_video` was clicked");

            let css_class = "suggested-action";

            // Check if the button currently has the `suggested-action` css class
            if button.has_css_class(css_class) {
                button.remove_css_class(css_class);
            } else {
                button.add_css_class(css_class);
            }
        });

        // Handle click on audio_input_microphone button
        self.audio_input_microphone.connect_clicked(move |button| {
            info!("Button `audio_input_microphone was clicked");

            let css_class = "suggested-action";

            // Check if button currently has the `suggested-action` css class
            if button.has_css_class(css_class) {
                button.remove_css_class(css_class);
            } else {
                button.add_css_class(css_class);
            }
        });

        // Handle click on call_stop button
        // self.call_stop
        //     .connect_clicked(clone!(@weak self as this => move |_| {
        //         info!("Button `call_stop` was clicked");

        //         // Hide call and show placeholder
        //         this.placeholder.set_visible(true);
        //         this.action_bar.set_visible(false);
        //         this.call_box.set_visible(false);

        //         // Empty the grid when stopping a call
        //         while let Some(child) = this.grid.child_at_index(0) {
        //             this.grid.remove(&child);
        //         }
        //     }));
        self.call_stop.connect_clicked(clone!(@weak self as this => move |_| {
            info!("Button `call_stop` was clicked");
            println!("Button `call_stop` was clicked");

            thread::spawn(|| {
                unsafe {
                    let ptr = ADAPTER.unwrap() as *mut c_void;

                    let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterHangupCall(ptr, CString::new(VIDEO_SENDER_ROUTE_ID.as_ref().unwrap().as_str()).unwrap().into_raw());

                    if !std::ffi::CStr::from_ptr(rv).to_str().unwrap().eq("") {
                        println!("Error in SaltpaneloAdapterHandupCall: {}", std::ffi::CStr::from_ptr(rv).to_str().unwrap());
                    }
                }
            });

            thread::spawn(|| {
                unsafe {
                    let ptr = ADAPTER.unwrap() as *mut c_void;

                    let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterHangupCall(ptr, CString::new(VIDEO_RECEIVER_ROUTE_ID.as_ref().unwrap().as_str()).unwrap().into_raw());

                    if !std::ffi::CStr::from_ptr(rv).to_str().unwrap().eq("") {
                        println!("Error in SaltpaneloAdapterHandupCall: {}", std::ffi::CStr::from_ptr(rv).to_str().unwrap());
                    }
                }
            });

            thread::spawn(|| {
                unsafe {
                    let ptr = ADAPTER.unwrap() as *mut c_void;

                    let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterHangupCall(ptr, CString::new(AUDIO_SENDER_ROUTE_ID.as_ref().unwrap().as_str()).unwrap().into_raw());

                    if !std::ffi::CStr::from_ptr(rv).to_str().unwrap().eq("") {
                        println!("Error in SaltpaneloAdapterHandupCall: {}", std::ffi::CStr::from_ptr(rv).to_str().unwrap());
                    }
                }
            });
            
            thread::spawn(|| {
                unsafe {
                    let ptr = ADAPTER.unwrap() as *mut c_void;

                    let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterHangupCall(ptr, CString::new(AUDIO_RECEIVER_ROUTE_ID.as_ref().unwrap().as_str()).unwrap().into_raw());

                    if !std::ffi::CStr::from_ptr(rv).to_str().unwrap().eq("") {
                        println!("Error in SaltpaneloAdapterHandupCall: {}", std::ffi::CStr::from_ptr(rv).to_str().unwrap());
                    }
                }
            });

            unsafe {
                glib::idle_add(move || {
                    WINDOW.as_ref().unwrap().call_pane().placeholder().set_visible(true);
                    WINDOW.as_ref().unwrap().call_pane().action_bar().set_visible(false);
                    WINDOW.as_ref().unwrap().call_pane().call_box().set_visible(false);
                    
                    while let Some(child) = WINDOW.as_ref().unwrap().call_pane().grid().child_at_index(0) {
                        WINDOW.as_ref().unwrap().call_pane().grid().remove(&child);
                    }

                    glib::Continue(false)
                });
            };
        }));
    }
}

impl WidgetImpl for CallPaneTemplate {}
impl BoxImpl for CallPaneTemplate {}
