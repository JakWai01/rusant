pub mod template;

use self::template::CallPaneTemplate;

use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct CallPane(ObjectSubclass<CallPaneTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for CallPane {
    fn default() -> Self {
        Self::new()
    }
}

impl CallPane {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
