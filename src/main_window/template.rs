use crate::{
    article_list::{ArticleList, template::ArticleListTemplate},
    feed_list::{FeedList, template::FeedListTemplate}
};

use super::MainWindow;

use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::{ObjectSubclass, ObjectSubclassExt},
        InitializingObject,
    },
    StaticTypeExt,
};
use gtk::{
    prelude::{GObjectPropertyExpressionExt, InitializingWidgetExt},
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate, Widget,
};
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow, Leaflet};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/main-window.ui")]
pub struct MainWindowTemplate {
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,

    #[template_child]
    pub feed_list: TemplateChild<FeedList>,

    #[template_child]
    pub article_list: TemplateChild<ArticleList>,
}

#[object_subclass]
impl ObjectSubclass for MainWindowTemplate {
    const NAME: &'static str = "MainWindow";

    type Type = MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
