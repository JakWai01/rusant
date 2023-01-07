pub mod template;

use crate::{rusant_call_pane::CallPane, rusant_contact_list::ContactList};

use self::template::ContactItemTemplate;

use gio::{subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt, ObjectImpl}, traits::ListModelExt};
use glib::{clone, closure, closure_local, wrapper, BindingFlags, ObjectExt, Cast};
use gtk::{
    ffi::gtk_check_button_get_active,
    traits::{ButtonExt, CheckButtonExt, WidgetExt},
    Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget,
};

wrapper! {
    pub struct ContactItem(ObjectSubclass<ContactItemTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl ContactItem {
    pub fn new(name: &str) -> Self {
        glib::Object::new(&[("name", &name)])
    }

    pub fn avatar(&self) -> libadwaita::Avatar {
        self.imp().avatar.get()
    }

    pub fn label(&self) -> gtk::Label {
        self.imp().label.get()
    }

    pub fn call(&self) -> gtk::Button {
        self.imp().call.get()
    }

    pub fn video_call(&self) -> gtk::Button {
        self.imp().video_call.get()
    }

    pub fn selection(&self) -> gtk::CheckButton {
        self.imp().selection.get()
    }

    pub fn handle_call_click(&self, call_pane: &CallPane) {
        let imp = ContactItemTemplate::from_instance(&self);
        imp.call
            .connect_clicked(clone!(@strong call_pane => move |_| {
                call_pane.call_box().set_visible(true);
                call_pane.placeholder().set_visible(false);
                call_pane.action_bar().set_visible(true);
            }));
    }

    pub fn handle_video_call_click(&self, call_pane: &CallPane) {
        let imp = ContactItemTemplate::from_instance(&self);
        imp.video_call
            .connect_clicked(clone!(@strong call_pane => move |_| {
                call_pane.call_box().set_visible(true);
                call_pane.placeholder().set_visible(false);
                call_pane.action_bar().set_visible(true);
            }));
    }
    
    pub fn handle_selection_toggle(&self, contact_list: &ContactList) {
        self.selection().connect_toggled(clone!(@strong self as this, @strong contact_list => move |_| {
            println!("CheckButton click");
            
            let mut position = 0;
            while let Some(item) = contact_list.contacts().item(position) {
                let contact_item = item.downcast_ref::<ContactItem>().expect("The object needs to be of type `ContactItem`.");

                if contact_item.get_name() == this.get_name() {
                    if contact_item.property::<bool>("active") == true {
                        contact_item.set_property("active", false);
                        contact_list.dec_n_selected();
                    } else {
                        contact_item.set_property("active", true);
                        contact_list.inc_n_selected();
                    }
                    break
                } else {
                    position += 1;
                } 
            }
            
            contact_list.title().set_title(format!("{} Selected", contact_list.get_n_selected()).as_str());
        }));
    }

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
}
