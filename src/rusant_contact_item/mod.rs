pub mod template;

use crate::rusant_call_pane::CallPane;

use self::template::ContactItemTemplate;

use gio::subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt};
use glib::{clone, wrapper};
use gtk::{
    traits::{ButtonExt, WidgetExt},
    Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget,
};

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

    pub fn  selection(&self) -> gtk::CheckButton {
        self.imp().selection.get()
    }

    pub fn handle_call_click(&self, call_pane: &CallPane) {
        let imp = ContactItemTemplate::from_instance(&self);
        imp.call
            .connect_clicked(clone!(@strong call_pane => move |_| {
                call_pane.call_box().set_visible(true);
                call_pane.placeholder().set_visible(false);
                call_pane.action_bar().set_visible(true);
            }));
    }

    pub fn handle_video_call_click(&self, call_pane: &CallPane) {
        let imp = ContactItemTemplate::from_instance(&self);
        imp.video_call
            .connect_clicked(clone!(@strong call_pane => move |_| {
                call_pane.call_box().set_visible(true);
                call_pane.placeholder().set_visible(false);
                call_pane.action_bar().set_visible(true);
            }));
    }

    pub fn enter_selection_mode(&self) {
        self.call().set_visible(false);
        self.video_call().set_visible(false);
        self.selection().set_visible(true);
    }

    pub fn leave_selection_mode(&self) {
        self.call().set_visible(true);
        self.video_call().set_visible(true);
        self.selection().set_visible(false);
    }
}
