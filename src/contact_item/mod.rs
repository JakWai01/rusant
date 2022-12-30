pub mod template;

use self::template::ContactItemTemplate;

use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct ContactItem(ObjectSubclass<ContactItemTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for ContactItem {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactItem {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
