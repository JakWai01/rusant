pub mod template;

use crate::{rusant_call_pane::CallPane, rusant_contact_list::ContactList};

use self::template::ContactItemTemplate;

use gio::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};
use glib::{clone, wrapper, BindingFlags, ObjectExt, closure_local, closure};
use gtk::{
    traits::{ButtonExt, WidgetExt, CheckButtonExt},
    Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget, ffi::gtk_check_button_get_active,
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

    pub fn handle_selection_toggle(&self) {
        let imp = ContactItemTemplate::from_instance(&self);
        self.set_property("active", true);
        imp.selection.connect_toggled(clone!(@weak self as this => move |_| {
            println!("Toggled");
            if this.property::<bool>("active") == true {
                this.set_property("active", false)
            } else {
                this.set_property("active", true)
            }

            println!("{:?}", this.property::<bool>("active"));
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
}
