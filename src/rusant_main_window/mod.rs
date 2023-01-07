mod template;

use template::MainWindowTemplate;

use glib::{wrapper, Cast, Object, ParamSpec};
use gtk::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window,
};
use libadwaita::Application;

wrapper! {
    pub struct MainWindow(ObjectSubclass<MainWindowTemplate>)
    @extends libadwaita::ApplicationWindow, ApplicationWindow, Window, Widget,
    @implements ActionGroup, ActionMap, Accessible, Buildable,
                ConstraintTarget, Native, Root, ShortcutManager;
}

impl MainWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)])
    }
}
