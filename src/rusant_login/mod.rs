mod template;

use crate::rusant_main_window::MainWindow;
use glib::{clone, Cast};
use gtk::{self, glib, subclass::prelude::*, traits::WidgetExt, CompositeTemplate};
use gtk_macros::spawn;
use template::LoginTemplate;

glib::wrapper! {
    pub struct Login(ObjectSubclass<LoginTemplate>)
        @extends gtk::Widget, libadwaita::Bin, @implements gtk::Accessible;
}

impl Login {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    fn parent_window(&self) -> MainWindow {
        self.root()
            .and_then(|root| root.downcast().ok())
            .expect("Login needs to have a parent window")
    }

    pub fn default_widget(&self) -> gtk::Widget {
        self.imp().next_button.get().upcast()
    }
}
