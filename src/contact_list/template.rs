use super::ContactPane;

use crate::contact_item::ContactItem;

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
    Box, CompositeTemplate,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-pane.ui")]
pub struct ContactPaneTemplate {
    #[template_child]
    pub contact_item: TemplateChild<ContactItem>,

    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,
}

#[object_subclass]
impl ObjectSubclass for ContactPaneTemplate {
    const NAME: &'static str = "ContactPane";

    type Type = ContactPane;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        ContactItem::ensure_type();

        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ContactPaneTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ContactPaneTemplate {}
impl BoxImpl for ContactPaneTemplate {}
