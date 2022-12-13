use super::MainWindow;
use crate::{article_item::ArticleItem, article_list::ArticleList, feed_item::FeedItem, feed_list::FeedList};

use glib::{
    object_subclass,
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
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate,
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
        FeedList::ensure_type();
        ArticleList::ensure_type();

        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        let feed_model = vec![
            FeedItem::new("The Verge", "https://www.theverge.com/rss/index.xml"),
            FeedItem::new("Ars Technica", "https://feeds.arstechnica.com/arstechnica/features"),
            FeedItem::new("Hacker News", "https://news.ycombinator.com/rss"),
        ];
        self.feed_list.set_model(feed_model);

        let article_model = vec![
            ArticleItem::new("The Verge - Article 1", "Article 1 summary placed in a handy label widget"),
            ArticleItem::new("The Verge - Article 2", "Article 2 summary placed in a handy label widget"),
            ArticleItem::new("The Verge - Article 3", "Article 3 summary placed in a handy label widget"),
            ArticleItem::new("The Verge - Article 4", "Article 4 summary placed in a handy label widget"),
        ];
        self.article_list.set_model(article_model);
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}