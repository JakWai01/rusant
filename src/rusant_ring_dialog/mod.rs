pub mod template;

use self::template::RingDialogTemplate;

use glib::wrapper;
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct RingDialog(ObjectSubclass<RingDialogTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for RingDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl RingDialog {
    /// Initialize a new RingDialog 
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
