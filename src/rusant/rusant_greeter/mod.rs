mod template;

use gio::{subclass::prelude::ObjectSubclassIsExt, traits::NetworkMonitorExt};
use glib::Cast;
use gtk::subclass::widget::CompositeTemplate;
use gtk::subclass::widget::WidgetClassSubclassExt;
use gtk::traits::WidgetExt;

use crate::rusant_main_window::MainWindow;
use template::GreeterTemplate;

glib::wrapper! {
    pub struct Greeter(ObjectSubclass<GreeterTemplate>)
        @extends gtk::Widget, libadwaita::Bin, @implements gtk::Accessible;
}

impl Greeter {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn default_widget(&self) -> gtk::Widget {
        self.imp().login_button.get().upcast()
    }

    fn parent_window(&self) -> MainWindow {
        self.root()
            .and_then(|root| root.downcast().ok())
            .expect("Login needs to have a parent window")
    }
}
