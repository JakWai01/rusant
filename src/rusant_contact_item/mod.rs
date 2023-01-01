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

    pub fn handle_call_click(&self, call_pane: &CallPane) {
        let imp = ContactItemTemplate::from_instance(&self);
        imp.call
            .connect_clicked(clone!(@strong call_pane => move |_| {
                println!("call click!");
                call_pane.call_box().set_visible(true);
                call_pane.placeholder().set_visible(false);
            }));
    }
}
