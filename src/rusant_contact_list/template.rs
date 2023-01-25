use super::ContactList;
use gio::traits::ListModelExt;
use glib::{clone, ParamFlags, ParamSpec, ParamSpecInt};
use glib::{Cast, ToValue, Value};
use gtk_macros::spawn;
use log::{debug, info};
use once_cell::sync::{Lazy, OnceCell};
use std::cell::Cell;

use crate::rusant_contact_item::ContactItem;

use libadwaita::{HeaderBar, WindowTitle};

use glib::{
    self, object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    StaticTypeExt,
};

use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    traits::{ButtonExt, WidgetExt},
    ActionBar, Box, Button, CompositeTemplate, ListBox, MenuButton, SearchEntry,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-list.ui")]
pub struct ContactListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub contacts_list: TemplateChild<ListBox>,

    #[template_child]
    pub selection_button: TemplateChild<Button>,

    #[template_child]
    pub action_bar: TemplateChild<ActionBar>,

    #[template_child]
    pub title: TemplateChild<WindowTitle>,

    #[template_child]
    pub select_cancel_button: TemplateChild<Button>,

    #[template_child]
    pub add_button: TemplateChild<Button>,

    #[template_child]
    pub menu: TemplateChild<MenuButton>,

    #[template_child]
    pub delete_button: TemplateChild<Button>,

    #[template_child]
    pub search_bar: TemplateChild<SearchEntry>,

    pub contacts: OnceCell<gio::ListStore>,

    selected: Cell<i32>,
}

#[object_subclass]
impl ObjectSubclass for ContactListTemplate {
    const NAME: &'static str = "ContactList";

    type Type = ContactList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        ContactItem::ensure_type();

        Self::bind_template(my_class);

        my_class.install_action("contacts.add", None, move |widget, _, _| {
            spawn!(clone!(@weak widget => async move {
                widget.show_add_contact_dialog().await;
            }));
        });
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ContactListTemplate {
    /// Construct a new ContactList
    fn constructed(&self) {
        self.parent_constructed();

        // Handle click on selection_button button
        self.selection_button
            .connect_clicked(clone!(@weak self as contact_list => move |_| {
                info!("Button selection_button was clicked");

                contact_list.action_bar.set_revealed(true);

                contact_list.add_button.set_visible(false);
                contact_list.title.set_title(format!("{} Selected", contact_list.selected.get()).as_str());
                contact_list.selection_button.set_visible(false);
                contact_list.menu.set_visible(false);

                contact_list.select_cancel_button.set_visible(true);
            }));

        // Handle click on select_cancel_button button
        self.select_cancel_button
            .connect_clicked(clone!(@weak self as contact_list => move |_| {
                info!("Button select_cancel_button was clicked");

                contact_list.action_bar.set_revealed(false);

                contact_list.add_button.set_visible(true);
                contact_list.title.set_title("Contacts");
                contact_list.selection_button.set_visible(true);
                contact_list.menu.set_visible(true);

                contact_list.select_cancel_button.set_visible(false);
            }));

        // Handle click on add_button button
        self.add_button.connect_clicked(move |button| {
            info!("Button add_button was clicked");

            button
                .activate_action("contacts.add", None)
                .expect("The action does not exist");
        });

        // Handle click on delete_button button
        self.delete_button.connect_clicked(clone!(@weak self as contact_list => move |button| {
            info!("Button delete_button was clicked");

            let contacts = contact_list.contacts.get().expect("`contacts` should be set in `set_model`.");

            let mut position = 0;

            // Iterate through all contacts
            while let Some(item) = contacts.item(position) {
                let contact_item = item.downcast_ref::<ContactItem>().expect("The object needs to be of type `ContactItem`.");

                // Check if the current contact is selected
                if contact_item.get_active() == true {
                    contacts.remove(position);
                    contact_list.selected.replace(contact_list.selected.take() - 1);

                    debug!("Removed contact: {}", contact_item.get_name());
                } else {
                    position += 1;
                }
            }

            contact_list.title.set_title(format!("{:?} Selected", contact_list.selected.take()).as_str());

            contact_list.action_bar.set_revealed(false);

            contact_list.add_button.set_visible(true);
            contact_list.title.set_title("Contacts");
            contact_list.selection_button.set_visible(true);
            contact_list.menu.set_visible(true);

            contact_list.select_cancel_button.set_visible(false);
        }));
    }

    /// Get properties defiend for ContactList
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecInt::new(
                "selected",
                "selected",
                "How many contacts are selected",
                0,
                65535,
                0,
                ParamFlags::READWRITE,
            )]
        });
        PROPERTIES.as_ref()
    }

    /// Set value for a given propery defined for ContactList
    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "selected" => {
                let selected = value.get().expect("The value needs to be of type 'i32'.");
                self.selected.replace(selected);
            }
            _ => unimplemented!(),
        }
    }

    /// Get value of a given property defined for ContactList
    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "selected" => {
                let result = self.selected.take();

                self.selected.set(result.clone());
                result.to_value()
            }
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ContactListTemplate {}
impl BoxImpl for ContactListTemplate {}
