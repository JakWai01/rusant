pub mod template;

use crate::{rusant_call_pane::CallPane, rusant_contact_item::ContactItem};

use self::template::ContactListTemplate;

use gio::{
    subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt},
    traits::ListModelExt,
    ListStore,
};
use glib::{clone, wrapper, Cast, ObjectExt, StaticType};
use glib::closure_local;
use gtk::{
    traits::{ButtonExt, EditableExt, GtkWindowExt, WidgetExt, CheckButtonExt},
    Accessible, Box, Buildable, ConstraintTarget, Orientable, SingleSelection, Widget,
};
use libadwaita::{prelude::MessageDialogExtManual, traits::MessageDialogExt};

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
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn set_model(&self, model: Vec<ContactItem>, call_pane: &CallPane) {
        let contacts = ListStore::new(ContactItem::static_type());

        self.imp().contacts.set(contacts.clone()).expect("Could not set contacts");

        for element in model {
            contacts.append(&element);
        }

        self.imp().contacts_list.bind_model(
            Some(&contacts),
            clone!(@strong call_pane, @strong self as this => move |x| {
                let name: String = x.property("name");

                let contact_item = ContactItem::new(&name);
                contact_item.handle_call_click(&call_pane);
                contact_item.handle_video_call_click(&call_pane);

                contact_item.avatar().set_text(Some(&name));
                contact_item.label().set_label(&name);

                let result = contact_item.ancestor(Widget::static_type());
                
                this.imp().selection_button.connect_clicked(clone!(@weak contact_item => move |_| {
                    contact_item.enter_selection_mode();
                }));

                this.imp().select_cancel_button.connect_clicked(clone!(@weak contact_item => move |_| {
                    contact_item.leave_selection_mode();
                }));

                // contact_item.selection().connect_toggled(|item| {
                //     println!("Toggled");
                // });

                contact_item.handle_selection_toggle();

                println!("{:?}", contact_item.selection().is_active());

                println!("{:?}", contact_item.n_bindings());
                result.unwrap()
            }),
        );
    }

    pub async fn show_add_contact_dialog(&self) {
        let builder =
            gtk::Builder::from_resource("/com/jakobwaibel/Rusant/rusant-contact-dialog.ui");
        let dialog = builder
            .object::<libadwaita::MessageDialog>("dialog")
            .unwrap();
        let entry = builder.object::<gtk::Entry>("entry").unwrap();

        entry.connect_changed(clone!(@weak self as obj, @weak dialog => move |entry| {
            let contact = entry.text();
            dialog.set_response_enabled("add", true);

            println!("{:?}", contact);
        }));

        dialog.set_transient_for(self.parent_window().as_ref());
        if dialog.run_future().await == "add" {
            println!("Add future result: {:?}", entry.text());
        };
    }

    /// Returns the parent GtkWindow containing this widget.
    fn parent_window(&self) -> Option<gtk::Window> {
        self.root()?.downcast().ok()
    }

    pub fn contacts(&self) -> gio::ListStore {
        self.imp()
            .contacts
            .get()
            .expect("`contacts` should be set in `setup_contacts`.")
            .clone()
    }

    // Returns ListBoxRow (create_collection_row)
    fn create_contact(&self, contact_item: &ContactItem) -> () {
        unimplemented!()
    }
}
