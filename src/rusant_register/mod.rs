mod template;

use glib::{Cast, clone};
use gtk::{self, glib, subclass::prelude::*, CompositeTemplate, traits::WidgetExt};
use gtk_macros::spawn;
use template::RegisterTemplate;

use crate::rusant_main_window::MainWindow;

glib::wrapper! {
    pub struct Register(ObjectSubclass<RegisterTemplate>)
        @extends gtk::Widget, libadwaita::Bin, @implements gtk::Accessible;
}

impl Register {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    fn parent_window(&self) -> MainWindow {
        self.root()
            .and_then(|root| root.downcast().ok())
            .expect("Register needs to have a parent window")
    }

    pub fn default_widget(&self) -> gtk::Widget {
        self.imp().next_button.get().upcast()
    }
}