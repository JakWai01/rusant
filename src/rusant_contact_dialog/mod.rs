pub mod template;

use self::template::ContactDialogTemplate;

use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct ContactDialog(ObjectSubclass<ContactDialogTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for ContactDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactDialog {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
