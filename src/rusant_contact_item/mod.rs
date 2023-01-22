pub mod template;

use std::{thread, os::raw::c_void, ffi::CString};

use crate::{receiver, rusant_call_pane::CallPane, rusant_contact_list::ContactList, sender, ADAPTER, ROUTE_ID, WINDOW};

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
                   
                    thread::spawn(|| {
                        unsafe { 
                            let ptr = ADAPTER.unwrap() as *mut c_void;

                            let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterRequestCall(ptr, CString::new("jean.doe@example.com").unwrap().into_raw(), CString::new("video").unwrap().into_raw());

                            if !std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
                                println!(
                                    "Error in SalpaneloAdapterRequestCall: {}",
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


                    if call_pane.action_bar().is_visible() {
                        info!("Button was clicked during a call! Please end the call before starting a new one.");
                    } else {
                    //     call_pane.call_box().set_visible(true);
                    //     call_pane.placeholder().set_visible(false);
                    //     call_pane.action_bar().set_visible(true);

                    //     spawn!(clone!(@weak this => async move {
                    //         this.show_ring_dialog().await;
                    //     }));

                    //     /*
                    //     * This part does not necessarily need to be here.
                    //     * It just has to be started once a call starts but this can be anywhere.
                    //     */
                    //     let sender = sender::VideoSenderPipeline::new("127.0.0.1", 3000);
                    //     sender.build();
                    //     sender.start();

                    //     let receiver = receiver::VideoReceiverPipeline::new("127.0.0.1", 3000);
                    //     let paintable = receiver.build();
                    //     receiver.start();

                    //     let picture = gtk::Picture::new();
                    //     picture.set_paintable(Some(&paintable));
                    //     picture.set_keep_aspect_ratio(true);

                    //     call_pane.grid().insert(&picture, 0);

                    //     let audio_sender = sender::AudioSenderPipeline::new("127.0.0.1", 3001);
                    //     audio_sender.build();
                    //     audio_sender.start();

                    //     let audio_receiver = receiver::AudioReceiverPipeline::new("127.0.0.1", 3001);
                    //     audio_receiver.build();
                    //     audio_receiver.start();

                        // call_pane.call_stop().connect_clicked(clone!(@weak call_pane => move |_| {
                        //     info!("Button `call_stop` was clicked");
                        //     println!("Button `call_stop` was clicked");

                        //     thread::spawn(|| {
                        //         unsafe {
                        //             let ptr = ADAPTER.unwrap() as *mut c_void;

                        //             let rv = saltpanelo_sys::saltpanelo::SaltpaneloAdapterHangupCall(ptr, CString::new(ROUTE_ID.as_ref().unwrap().as_str()).unwrap().into_raw());

                        //             if !std::ffi::CStr::from_ptr(rv).to_str().unwrap().eq("") {
                        //                 println!("Error in SaltpaneloAdapterHandupCall: {}", std::ffi::CStr::from_ptr(rv).to_str().unwrap());
                        //             }

                        //             glib::idle_add(move || {
                        //                 WINDOW.as_ref().unwrap().call_pane().placeholder().set_visible(true);
                        //                 WINDOW.as_ref().unwrap().call_pane().action_bar().set_visible(false);
                        //                 WINDOW.as_ref().unwrap().call_pane().call_box().set_visible(false);
                                        
                        //                 while let Some(child) = WINDOW.as_ref().unwrap().call_pane().grid().child_at_index(0) {
                        //                     WINDOW.as_ref().unwrap().call_pane().grid().remove(&child);
                        //                 }

                        //                 glib::Continue(false)
                        //             });
                        //         }
                        //     });

                        //     // Hide call and show placeholder
                        //     // call_pane.placeholder().set_visible(true);
                        //     // call_pane.action_bar().set_visible(false);
                        //     // call_pane.call_box().set_visible(false);

                        //     // // Empty the grid when stopping a call
                        //     // while let Some(child) = call_pane.grid().child_at_index(0) {
                        //     //     call_pane.grid().remove(&child);
                        //     // }

                        //     // audio_receiver.stop();
                        //     // receiver.stop();
                        //     // sender.stop();
                        //     // audio_sender.stop();
                        // }));
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

                call_pane.call_box().set_visible(true);
                call_pane.placeholder().set_visible(false);
                call_pane.action_bar().set_visible(true);
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
