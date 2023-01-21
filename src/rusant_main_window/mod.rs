mod template;

use std::{fs::File, os::raw::c_void, };

use gio::subclass::prelude::ObjectSubclassIsExt;
use template::MainWindowTemplate;

use glib::{wrapper, Cast, Object};
use gtk::{
    gio::{ActionGroup, ActionMap},
    traits::GtkWindowExt,
    Accessible, ApplicationWindow, Buildable, ConstraintTarget, Native, Root, ShortcutManager,
    Widget, Window,
};
use libadwaita::Application;

use crate::{rusant_contact_item::{ContactData, ContactItem}, rusant_greeter::Greeter, rusant_call_pane::CallPane, rusant_contact_list::ContactList};

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
            let backup_data: Vec<ContactData> = serde_json::from_reader(file)
                .expect("It should be possible to read `backup_data` from the json file.");

            // Convert `Vec<ContactData>` to `Vec<ContactItem>`
            let contacts: Vec<ContactItem> = backup_data
                .into_iter()
                .map(ContactItem::from_contact_data)
                .collect();

            contacts
        } else {
            Vec::new()
        }
    }

    pub fn leaflet(&self) -> libadwaita::Leaflet {
        self.imp().leaflet.get()
    }

    pub fn greeter(&self) -> Greeter {
        self.imp().greeter.get()
    }

    pub fn contact(&self) -> ContactList {
        self.imp().contact_list.get()
    }

    pub fn call_pane(&self) -> CallPane {
        self.imp().call_pane.get()
    }
}
