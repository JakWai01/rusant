use super::MainWindow;
use crate::{article_item::ArticleItem, article_list::ArticleList, feed_item::FeedItem, feed_list::FeedList};

use glib::{self, clone, MainContext, Continue, PRIORITY_DEFAULT, ObjectExt};

use curio::prelude::Request;
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
use rss::Channel;
use std::thread;

enum Message {
    UpdateArticleList(Channel),
    FeedSelected(String, String),
}

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

        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        let feed_model = vec![
            FeedItem::new("The Verge", "https://www.theverge.com/rss/index.xml"),
            FeedItem::new("Ars Technica", "https://feeds.arstechnica.com/arstechnica/features"),
            FeedItem::new("Hacker News", "https://news.ycombinator.com/rss"),
        ];
        self.feed_list.set_model(feed_model.clone());

        let article_list_clone= self.article_list.clone();
        let sender_clone = sender.clone();
        receiver.attach(None, move |x| match x {
            Message::UpdateArticleList(data) => {
                let mut article_model = vec![];

                for item in data.items() {
                    article_model.push(ArticleItem::new(&item.title.clone().unwrap(), &item.description.clone().unwrap()));
                }

                article_list_clone.set_model(article_model);
                return Continue(true);
            }
            Message::FeedSelected(_name, url) => {
                let sender_clone = sender.clone();

                thread::spawn(move || {
                    let response = Request::get(&url).send().unwrap();
                    let body = response.body.unwrap();
                    let rss_content = body.as_bytes();
                    let rss_channel = Channel::read_from(&rss_content[..]);

                    let _ = sender_clone.send(Message::UpdateArticleList(rss_channel.unwrap()));
                });

                return Continue(true);
            }
        });

        let feed_model_clone = feed_model.clone();
        self.feed_list.connect_local(
            "changed",
            false,
            clone!(@strong feed_model_clone => move |values| {
                let value: String = values[1].get().unwrap();
                let selection = feed_model_clone.iter().find(|x| x.property::<String>("name") == value).unwrap();
                let feed_name: String = selection.property::<String>("name");
                let feed_url: String = selection.property::<String>("url");

                let _ = sender_clone.send(Message::FeedSelected(feed_name, feed_url));
                None
            }),
        );
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}