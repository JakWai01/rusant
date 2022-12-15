use std::rc::Rc;

use super::MainWindow;
use crate::{article_item::ArticleItem, article_list::ArticleList, feed_item::FeedItem, feed_list::FeedList};

use glib::{self, clone, ObjectExt};

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
        self.feed_list.set_model(feed_model.clone());

        let feed_model_rt = Rc::new(feed_model.clone());
        let article_list_rt = Rc::new(self.article_list.clone());

        self.feed_list.connect_local(
            "changed",
            false,
            clone!(@strong feed_model_rt, @strong article_list_rt => move |values| {
                let value: String = values[1].get().unwrap();
                let selection = feed_model_rt.iter().find(|x| x.property::<String>("name") == value).unwrap();

                let feed_name:String = selection.property::<String>("name");
                let feed_url:String = selection.property::<String>("url");

                let article_model = vec![
                    ArticleItem::new(&format!("{} - Article 1", feed_name), &format!("Article 1 from {}", feed_url)),
                    ArticleItem::new(&format!("{} - Article 2", feed_name), &format!("Article 2 from {}", feed_url)),
                    ArticleItem::new(&format!("{} - Article 3", feed_name), &format!("Article 3 from {}", feed_url)),
                    ArticleItem::new(&format!("{} - Article 4", feed_name), &format!("Article 4 from {}", feed_url)),
                ];
                article_list_rt.set_model(article_model);

                None
            }),
        );
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}