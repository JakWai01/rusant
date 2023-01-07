pub mod template;

use crate::{rusant_call_pane::CallPane, rusant_contact_item::ContactItem};

use self::template::ContactListTemplate;

use gio::{
    subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt},
    traits::ListModelExt,
    ListStore,
};
use glib::closure_local;
use glib::{clone, wrapper, Cast, ObjectExt, StaticType};
use gtk::{
    traits::{ButtonExt, CheckButtonExt, EditableExt, GtkWindowExt, WidgetExt},
    Accessible, Box, Buildable, ConstraintTarget, Orientable, SingleSelection, Widget,
};
use libadwaita::{prelude::MessageDialogExtManual, traits::MessageDialogExt, WindowTitle};
use once_cell::sync::OnceCell;

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
        
        self.imp()
            .contacts
            .set(contacts.clone())
            .expect("Could not set contacts");
        
        for element in model {
            contacts.append(&element);
        }

        self.imp().contacts_list.bind_model(
            Some(&contacts),
            // Constructor for new contacts
            clone!(@strong call_pane, @weak self as this, @weak contacts => @default-panic, move |x| {
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

                this.imp().delete_button.connect_clicked(clone!(@weak contact_item => move |_| {
                    contact_item.leave_selection_mode();
                }));

                this.imp().call_button.connect_clicked(clone!(@weak contact_item => move |_| {
                    contact_item.leave_selection_mode();
                }));

                contact_item.handle_selection_toggle(&this);

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

            self.contacts().append(&ContactItem::new(&entry.text()));

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

    pub fn inc_n_selected(&self) -> i32{
        self.set_property("selected", self.property::<i32>("selected") + 1);
        self.property("selected")
    }

    pub fn dec_n_selected(&self) -> i32 {
        self.set_property("selected", self.property::<i32>("selected") - 1);
        self.property("selected")
    }

    pub fn get_n_selected(&self) -> i32 {
        self.property("selected")
    }

    pub fn title(&self) -> WindowTitle {
        self.imp().title.get()
    }
}
