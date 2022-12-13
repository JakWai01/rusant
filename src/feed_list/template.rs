use super::FeedList;

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
use libadwaita::HeaderBar;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/feed-list.ui")]
pub struct FeedListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,
}

#[object_subclass]
impl ObjectSubclass for FeedListTemplate {
    const NAME: &'static str = "FeedList";

    type Type = FeedList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for FeedListTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for FeedListTemplate {}
impl BoxImpl for FeedListTemplate {}