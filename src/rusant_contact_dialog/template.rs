
use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    }
};

use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, WidgetImpl},
        widget::CompositeTemplate,
    },
    Box, CompositeTemplate
};

use super::ContactDialog;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-dialog.ui")]
pub struct ContactDialogTemplate{}

#[object_subclass]
impl ObjectSubclass for ContactDialogTemplate {
    const NAME: &'static str = "ContactDialog";

    type Type = ContactDialog;

    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ContactDialogTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ContactDialogTemplate {}
impl BoxImpl for ContactDialogTemplate {}
