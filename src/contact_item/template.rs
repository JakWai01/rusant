use super::ContactItem;

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
        prelude::{BoxImpl, WidgetImpl},
        widget::CompositeTemplate,
    },
    Box, CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-item.ui")]
pub struct ContactItemTemplate {}

#[object_subclass]
impl ObjectSubclass for ContactItemTemplate {
    const NAME: &'static str = "ContactItem";

    type Type = ContactItem;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ContactItemTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ContactItemTemplate {}
impl BoxImpl for ContactItemTemplate {}
