pub mod template;

use std::{thread, os::raw::c_void, ffi::CString, collections::HashSet};

use crate::{receiver, rusant_call_pane::CallPane, rusant_contact_list::ContactList, sender, ADAPTER, WINDOW, REQUESTED_VIDEO_SENDER, REQUESTED_VIDEO_RECEIVER, REQUESTED_AUDIO_SENDER, REQUESTED_AUDIO_RECEIVER, REQUESTED_ONLY_AUDIO_SENDER, REQUESTED_ONLY_AUDIO_RECEIVER};

use self::template::ContactItemTemplate;

use gio::{
    subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt},
    traits::ListModelExt,
};
use glib::{clone, wrapper, Cast, ObjectExt};
use gst::traits::ElementExt;
use gtk::{
    traits::{ButtonExt, CheckButtonExt, GtkWindowExt, WidgetExt},
    Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget,
};
use gtk_macros::spawn;
use libadwaita::{prelude::MessageDialogExtManual, traits::MessageDialogExt};
use log::{debug, info};
use serde::{Deserialize, Serialize};

wrapper! {
    pub struct ContactItem(ObjectSubclass<ContactItemTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl ContactItem {
    /// Initialize a new ContactItem
    pub fn new(name: &str) -> Self {
        glib::Object::new(&[("name", &name)])
    }

    /// Get avatar widget
    pub fn avatar(&self) -> libadwaita::Avatar {
        self.imp().avatar.get()
    }

    /// Get label widget
    pub fn label(&self) -> gtk::Label {
        self.imp().label.get()
    }

    /// Get call widget
    pub fn call(&self) -> gtk::Button {
        self.imp().call.get()
    }

    /// Get video_call widget
    pub fn video_call(&self) -> gtk::Button {
        self.imp().video_call.get()
    }

    /// Get selection widget
    pub fn selection(&self) -> gtk::CheckButton {
        self.imp().selection.get()
    }

    /// Handle click on handle_call_click button
    pub fn handle_call_click(&self, call_pane: &CallPane) {
        let imp = ContactItemTemplate::from_instance(&self);

        imp.call
            .connect_clicked(clone!(@strong call_pane, @weak self as this => move |_| {
                    info!("Button call was clicked");
                   
                if call_pane.action_bar().is_visible() {
                    info!("Button was clicked during a call! Please end the call before starting a new one.");
                } else {
                    thread::spawn(|| {
                        unsafe { 
                            REQUESTED_ONLY_AUDIO_SENDER = true;

                            let ptr = ADAPTER.unwrap() as *mut c_void;
                            
                            let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterRequestCall(ptr, CString::new("jean.doe@example.com").unwrap().into_raw(), CString::new("ONLY_AUDIO_SENDER").unwrap().into_raw());

                            if !std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
                                println!(
                                    "Error in SaltpaneloAdapterRequestCall: {}",
                                    std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap()
                                );
                            }

                            if rv.r0 == 1 {
                                println!("Callee accepted the call");
                            } else {
                                println!("Callee denied the call");
                            }
                        };
                    });

                    thread::spawn(|| {
                        unsafe { 
                            REQUESTED_ONLY_AUDIO_RECEIVER = true;

                            let ptr = ADAPTER.unwrap() as *mut c_void;
                            
                            let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterRequestCall(ptr, CString::new("jean.doe@example.com").unwrap().into_raw(), CString::new("ONLY_AUDIO_RECEIVER").unwrap().into_raw());

                            if !std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
                                println!(
                                    "Error in SaltpaneloAdapterRequestCall: {}",
                                    std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap()
                                );
                            }

                            if rv.r0 == 1 {
                                println!("Callee accepted the call");
                            } else {
                                println!("Callee denied the call");
                            }
                        };
                    });
                }
                    
                }));
    }

    pub async fn show_ring_dialog(&self) {
        info!("Showing ring dialog");

        let builder = gtk::Builder::from_resource("/com/jakobwaibel/Rusant/rusant-ring-dialog.ui");

        let dialog = builder
            .object::<libadwaita::MessageDialog>("dialog")
            .unwrap();

        dialog.set_transient_for(self.parent_window().as_ref());

        // dialog.set_response_enabled("accept", true);

        if dialog.run_future().await == "accept" {
            debug!("Accepting call");

            println!("Accepting the call");
        }
    }

    /// Returns the parent GtkWindow containing this widget.
    fn parent_window(&self) -> Option<gtk::Window> {
        self.root()?.downcast().ok()
    }

    /// Handle click on handle_video_call_click button
    pub fn handle_video_call_click(&self, call_pane: &CallPane) {
        let imp = ContactItemTemplate::from_instance(&self);

        imp.video_call
            .connect_clicked(clone!(@strong call_pane => move |_| {
                info!("Button video_call was clicked");
                    
                if call_pane.action_bar().is_visible() {
                    info!("Button was clicked during a call! Please end the call before starting a new one.");
                } else {
                    thread::spawn(|| {
                        unsafe { 
                            REQUESTED_VIDEO_SENDER = true;

                            let ptr = ADAPTER.unwrap() as *mut c_void;
                            
                            let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterRequestCall(ptr, CString::new("jean.doe@example.com").unwrap().into_raw(), CString::new("VIDEO_SENDER").unwrap().into_raw());

                            if !std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
                                println!(
                                    "Error in SaltpaneloAdapterRequestCall: {}",
                                    std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap()
                                );
                            }

                            if rv.r0 == 1 {
                                println!("Callee accepted the call");
                            } else {
                                println!("Callee denied the call");
                            }
                        };
                    });

                    thread::spawn(|| {
                        unsafe { 
                            REQUESTED_VIDEO_RECEIVER = true;

                            let ptr = ADAPTER.unwrap() as *mut c_void;
                            
                            let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterRequestCall(ptr, CString::new("jean.doe@example.com").unwrap().into_raw(), CString::new("VIDEO_RECEIVER").unwrap().into_raw());

                            if !std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
                                println!(
                                    "Error in SaltpaneloAdapterRequestCall: {}",
                                    std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap()
                                );
                            }

                            if rv.r0 == 1 {
                                println!("Callee accepted the call");
                            } else {
                                println!("Callee denied the call");
                            }
                        };
                    });
                    
                    thread::spawn(|| {
                        unsafe { 
                            REQUESTED_AUDIO_SENDER = true;

                            let ptr = ADAPTER.unwrap() as *mut c_void;
                            
                            let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterRequestCall(ptr, CString::new("jean.doe@example.com").unwrap().into_raw(), CString::new("AUDIO_SENDER").unwrap().into_raw());

                            if !std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
                                println!(
                                    "Error in SaltpaneloAdapterRequestCall: {}",
                                    std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap()
                                );
                            }

                            if rv.r0 == 1 {
                                println!("Callee accepted the call");
                            } else {
                                println!("Callee denied the call");
                            }
                        };
                    });

                    thread::spawn(|| {
                        unsafe { 
                            REQUESTED_AUDIO_RECEIVER = true;

                            let ptr = ADAPTER.unwrap() as *mut c_void;
                            
                            let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterRequestCall(ptr, CString::new("jean.doe@example.com").unwrap().into_raw(), CString::new("AUDIO_RECEIVER").unwrap().into_raw());

                            if !std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
                                println!(
                                    "Error in SaltpaneloAdapterRequestCall: {}",
                                    std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap()
                                );
                            }

                            if rv.r0 == 1 {
                                println!("Callee accepted the call");
                            } else {
                                println!("Callee denied the call");
                            }
                        };
                    });
                }
            }));
    }

    /// Handle toggle on selection CheckBox  
    pub fn handle_selection_toggle(&self, contact_list: &ContactList) {
        self.selection().connect_toggled(clone!(@strong self as this, @strong contact_list => move |_| {
            info!("Button `handle_selection_toggle` was toggled");

            let mut position = 0;

            // Iterate through all contacts and check if the contact is selected
            while let Some(item) = contact_list.contacts().item(position) {
                let contact_item = item.downcast_ref::<ContactItem>().expect("The object needs to be of type `ContactItem`.");

                // Compare contacts by name
                if contact_item.get_name() == this.get_name() {
                    // Check if contact is currenlty selected
                    if contact_item.property::<bool>("active") == true {
                        debug!("Contact {} was just selected", contact_item.get_name());

                        contact_item.set_property("active", false);

                        // Decrement n_selected in order to represent the number of selected contacts
                        contact_list.dec_n_selected();
                    } else {
                        debug!("Contact {} was just unselected", contact_item.get_name());

                        contact_item.set_property("active", true);

                        // Increment n_selected in order to represent the number of selected contacts
                        contact_list.inc_n_selected();
                    }
                    break
                } else {
                    position += 1;
                }
            }

            // Adjust the title to represent the number of selected contacts
            contact_list.title().set_title(format!("{} Selected", contact_list.get_n_selected()).as_str());

            debug!("Updated `title` of `contact_list` to: {}", contact_list.get_n_selected());
        }));
    }

    /// Hide/Show certain widgets when entering selection mode
    pub fn enter_selection_mode(&self) {
        self.call().set_visible(false);
        self.video_call().set_visible(false);
        self.selection().set_visible(true);
    }

    pub fn leave_selection_mode(&self) {
        self.call().set_visible(true);
        self.video_call().set_visible(true);
        self.selection().set_visible(false);
    }

    pub fn n_bindings(&self) -> i32 {
        self.imp().bindings.borrow().len().try_into().unwrap()
    }

    pub fn get_active(&self) -> bool {
        self.property::<bool>("active")
    }

    pub fn get_name(&self) -> String {
        self.property("name")
    }

    pub fn to_contact_data(&self) -> ContactData {
        let name = self.property("name");

        ContactData { name }
    }

    pub fn from_contact_data(contact_data: ContactData) -> Self {
        let name = contact_data.name;

        Self::new(&name)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ContactData {
    pub name: String,
}
