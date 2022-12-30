use super::ContactList;

use crate::rusant_contact_item::ContactItem;

use libadwaita::HeaderBar;

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
    Box, CompositeTemplate, ListBox
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-list.ui")]
pub struct ContactListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub list_box: TemplateChild<ListBox>,
}

#[object_subclass]
impl ObjectSubclass for ContactListTemplate {
    const NAME: &'static str = "ContactList";

    type Type = ContactList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        ContactItem::ensure_type();

        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ContactListTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ContactListTemplate {}
impl BoxImpl for ContactListTemplate {}
