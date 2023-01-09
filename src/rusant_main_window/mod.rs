mod template;

use gio::subclass::prelude::ObjectSubclassIsExt;
use template::MainWindowTemplate;

use glib::{wrapper, Object, Cast};
use gtk::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window, traits::GtkWindowExt,
};
use libadwaita::Application;

wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowTemplate>)
    @extends libadwaita::ApplicationWindow, ApplicationWindow, Window, Widget,
    @implements ActionGroup, ActionMap, Accessible, Buildable,
                ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainWindow {
    /// Initialize a new MainWindow
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)])
    }

    pub fn switch_to_login_page(&self) {
        let imp = self.imp();
        imp.main_stack.set_visible_child(&*imp.login);
    }

    pub fn switch_to_greeter_page(&self) {
        let imp = self.imp();
        imp.main_stack.set_visible_child(&*imp.greeter);
    }

    pub fn switch_to_leaflet(&self) {
        let imp = self.imp();
        imp.main_stack.set_visible_child(&*imp.leaflet);
    }

    pub fn switch_to_register_page(&self) {
        let imp = self.imp();
        imp.main_stack.set_visible_child(&*imp.register);
    }
}
