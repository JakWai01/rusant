mod template;

use std::fs::File;

use gio::subclass::prelude::ObjectSubclassIsExt;
use template::MainWindowTemplate;

use glib::{wrapper, Object, Cast};
use gtk::{
    gio::{ActionGroup, ActionMap},
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window, traits::GtkWindowExt,
};
use libadwaita::Application;

use crate::rusant_contact_item::{ContactData, ContactItem};

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

    pub fn switch_to_greeter_page(&self) {
        let imp = self.imp();
        imp.main_stack.set_visible_child(&*imp.greeter);
    }

    pub fn switch_to_leaflet(&self) {
        let imp = self.imp();
        imp.main_stack.set_visible_child(&*imp.leaflet);
    }

    fn restore_data(&self) -> Vec<ContactItem> {
        if let Ok(file) = File::open("data.json") {
            // Deserialize data from file to vector
            let backup_data: Vec<ContactData> = serde_json::from_reader(file).expect("It should be possible to read `backup_data` from the json file.");
            
            // Convert `Vec<ContactData>` to `Vec<ContactItem>`
            let contacts: Vec<ContactItem> = backup_data.into_iter().map(ContactItem::from_contact_data).collect();

            contacts
        } else {
            Vec::new()
        }
    }
}
