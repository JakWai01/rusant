mod template;

use template::CallWindowTemplate;

use glib::{wrapper, Object};
use gtk::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window,
};
use libadwaita::Application;

wrapper! {
    pub struct CallWindow(ObjectSubclass<CallWindowTemplate>)
    @extends ApplicationWindow, Window, Widget,
    @implements ActionGroup, ActionMap, Accessible, Buildable,
                ConstraintTarget, Native, Root, ShortcutManager;
}

impl CallWindow {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)])
    }
}
