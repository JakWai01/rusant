mod template;

use template::MainWindowTemplate;

use glib::{wrapper, Object};
use gtk::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window
};
use adw::Application;

wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowTemplate>)
    @extends ApplicationWindow, Window, Widget,
    @implements ActionGroup, ActionMap, Accessible, Buildable,
                ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)])
    }
}