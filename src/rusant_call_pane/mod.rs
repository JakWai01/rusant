pub mod template;

use self::template::CallPaneTemplate;

use gio::subclass::prelude::ObjectSubclassIsExt;
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

    pub fn placeholder(&self) -> libadwaita::StatusPage {
        self.imp().placeholder.get()
    }

    pub fn call_box(&self) -> gtk::Box {
        self.imp().call_box.get()
    }

    pub fn action_bar(&self) -> gtk::ActionBar {
        self.imp().action_bar.get()
    }
}
