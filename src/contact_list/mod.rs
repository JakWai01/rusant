pub mod template;

use self::template::ContactPaneTemplate;

use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct ContactPane(ObjectSubclass<ContactPaneTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for ContactPane {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactPane {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
