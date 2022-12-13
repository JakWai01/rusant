use super::ArticleList;

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
    Box, Button, CompositeTemplate,
};
use libadwaita::HeaderBar;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/article-list.ui")]
pub struct ArticleListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub back_button: TemplateChild<Button>,
}

#[object_subclass]
impl ObjectSubclass for ArticleListTemplate {
    const NAME: &'static str = "ArticleList";

    type Type = ArticleList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ArticleListTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArticleListTemplate {}
impl BoxImpl for ArticleListTemplate {}