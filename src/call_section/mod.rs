pub mod template;

use self::template::CallSectionTemplate;

use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct CallSection(ObjectSubclass<CallSectionTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for CallSection {
    fn default() -> Self {
        Self::new()
    }
}

impl CallSection {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}