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
    /// Initialize new CallPane
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    /// Get the placeholder widget
    pub fn placeholder(&self) -> libadwaita::StatusPage {
        self.imp().placeholder.get()
    }

    /// Get the call_box widget
    pub fn call_box(&self) -> gtk::Box {
        self.imp().call_box.get()
    }

    /// Get the action_bar widget
    pub fn action_bar(&self) -> gtk::ActionBar {
        self.imp().action_bar.get()
    }

    pub fn grid(&self) -> gtk::FlowBox {
        self.imp().grid.get()
    }

    pub fn call_stop(&self) -> gtk::Button {
        self.imp().call_stop.get()
    }
}
