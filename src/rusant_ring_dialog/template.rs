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

use super::RingDialog;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-ring-dialog.ui")]
pub struct RingDialogTemplate {}

#[object_subclass]
impl ObjectSubclass for RingDialogTemplate {
    const NAME: &'static str = "RingDialog";

    type Type = RingDialog;

    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for RingDialogTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for RingDialogTemplate {}
impl BoxImpl for RingDialogTemplate {}
