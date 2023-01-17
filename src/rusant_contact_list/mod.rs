pub mod template;

use crate::{rusant_call_pane::CallPane, rusant_contact_item::ContactItem};

use self::template::ContactListTemplate;

use gio::{subclass::prelude::ObjectSubclassIsExt, ListStore};
use glib::{clone, wrapper, Cast, ObjectExt, StaticType};
use gtk::{
    traits::{ButtonExt, EditableExt, GtkWindowExt, WidgetExt},
    Accessible, Box, Buildable, ConstraintTarget, Orientable, SearchEntry, Widget,
};
use libadwaita::{prelude::MessageDialogExtManual, traits::MessageDialogExt, WindowTitle};
use log::{debug, info};

wrapper! {
    pub struct ContactList(ObjectSubclass<ContactListTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for ContactList {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactList {
    /// Initialize a new ContactList
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    /// Initialize elements of the ContactList
    pub fn set_model(&self, model: Vec<ContactItem>, call_pane: &CallPane) {
        let contacts = ListStore::new(ContactItem::static_type());

        self.imp()
            .contacts
            .set(contacts.clone())
            .expect("Could not set contacts");

        // Add contacts specified in the model argument to contacts
        for element in model {
            contacts.append(&element);
        }

        // Constructor for new item in contacts
        self.imp().contacts_list.bind_model(
            Some(&contacts),
            clone!(@strong call_pane, @weak self as this, @weak contacts => @default-panic, move |x| {
                let name: String = x.property("name");

                let contact_item = ContactItem::new(&name);

                contact_item.handle_call_click(&call_pane);
                contact_item.handle_video_call_click(&call_pane);

                // Set name property of avatar and label
                contact_item.avatar().set_text(Some(&name));
                contact_item.label().set_label(&name);

                let result = contact_item.ancestor(Widget::static_type());

                // Handle click on selection_button
                this.imp().selection_button.connect_clicked(clone!(@weak contact_item => move |_| {
                    info!("Button selection_button was clicked");

                    contact_item.enter_selection_mode();
                }));

                // Handle click on select_cancel_button
                this.imp().select_cancel_button.connect_clicked(clone!(@weak contact_item => move |_| {
                    info!("Button select_cancel_button was clicked");

                    contact_item.leave_selection_mode();
                }));

                // Handle click on delete_button button
                this.imp().delete_button.connect_clicked(clone!(@weak contact_item => move |_| {
                    info!("Button delete_button was clicked");

                    contact_item.leave_selection_mode();
                }));

                // Handle click on call_button button
                // this.imp().call_button.connect_clicked(clone!(@weak contact_item, @weak this, @weak call_pane => move |_| {
                //     info!("Button call_button was clicked");

                //     contact_item.leave_selection_mode();
                // }));
                
                // Handle contact search
                this.imp().search_bar.connect_search_changed(clone!(@weak this, @weak contact_item, @strong name => move |entry| {
                    debug!("Search changed: {}", entry.text());

                    if name.to_lowercase().contains(entry.text().as_str().to_lowercase().as_str()) {
                        debug!("Found match: {}", name);

                        contact_item.set_visible(true);
                    } else {
                        contact_item.set_visible(false);
                    }
                }));

                // this.handle_call_button_click(&call_pane);

                contact_item.handle_selection_toggle(&this);

                result.unwrap()
            }),
        );
    }

    /// Handle dialog that shows up when creating a new contact
    pub async fn show_add_contact_dialog(&self) {
        info!("Showing dialog to add new contact");

        let builder =
            gtk::Builder::from_resource("/com/jakobwaibel/Rusant/rusant-contact-dialog.ui");

        let dialog = builder
            .object::<libadwaita::MessageDialog>("dialog")
            .unwrap();

        let entry = builder.object::<gtk::Entry>("entry").unwrap();

        // Connect to changed signal
        entry.connect_changed(clone!(@weak self as obj, @weak dialog => move |entry| {
            dialog.set_response_enabled("add", true);
        }));

        dialog.set_transient_for(self.parent_window().as_ref());

        // Handle click on the add button contained in the dialog
        if dialog.run_future().await == "add" {
            debug!("Adding new contact: {}", entry.text());

            self.contacts().append(&ContactItem::new(&entry.text()));
        };
    }

    /// Returns the parent GtkWindow containing this widget.
    fn parent_window(&self) -> Option<gtk::Window> {
        self.root()?.downcast().ok()
    }

    /// Get contacts
    pub fn contacts(&self) -> gio::ListStore {
        self.imp()
            .contacts
            .get()
            .expect("`contacts` should be set in `setup_contacts`.")
            .clone()
    }

    /// Increment n_selected by one
    pub fn inc_n_selected(&self) -> i32 {
        self.set_property("selected", self.property::<i32>("selected") + 1);
        self.property("selected")
    }

    /// Decrement n_selected by one
    pub fn dec_n_selected(&self) -> i32 {
        self.set_property("selected", self.property::<i32>("selected") - 1);
        self.property("selected")
    }

    /// Get the current value of n_selected
    pub fn get_n_selected(&self) -> i32 {
        self.property("selected")
    }

    /// Get title widget
    pub fn title(&self) -> WindowTitle {
        self.imp().title.get()
    }

    /// Handle click on call_button button
    // pub fn handle_call_button_click(&self, call_pane: &CallPane) {
    //     self.imp().call_button.connect_clicked(
    //         clone!(@weak self as contact_list, @weak call_pane => move |_| {
    //             info!("Button call_button was clicked");

    //             contact_list.imp().action_bar.set_revealed(false);

    //             contact_list.imp().add_button.set_visible(true);
    //             contact_list.imp().title.set_title("Contacts");
    //             contact_list.imp().selection_button.set_visible(true);
    //             contact_list.imp().menu.set_visible(true);

    //             contact_list.imp().select_cancel_button.set_visible(false);

    //             call_pane.call_box().set_visible(true);
    //             call_pane.placeholder().set_visible(false);
    //             call_pane.action_bar().set_visible(true);
    //         }),
    //     );
    // }

    pub fn search_bar(&self) -> SearchEntry {
        self.imp().search_bar.get()
    }
}