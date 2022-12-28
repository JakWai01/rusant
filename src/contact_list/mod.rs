pub mod template;

use self::template::ContactListTemplate;

use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct ContactList(ObjectSubclass<ContactListTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for ContactList {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactList {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}