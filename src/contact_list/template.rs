use super::ContactList;

use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    Box, CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/contact-list.ui")]
pub struct ContactListTemplate {
}

#[object_subclass]
impl ObjectSubclass for ContactListTemplate {
    const NAME: &'static str = "ContactList";

    type Type = ContactList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
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