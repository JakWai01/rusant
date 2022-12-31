pub mod template;

use self::template::ContactItemTemplate;

use gio::subclass::prelude::ObjectSubclassIsExt;
use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

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
}
