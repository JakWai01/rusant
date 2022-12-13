mod template;

use glib::Object;
use template::ArticleItemTemplate;

glib::wrapper! {
    pub struct ArticleItem(ObjectSubclass<ArticleItemTemplate>);
}

impl ArticleItem {
    pub fn new(title: &str, summary: &str) -> Self {
        Object::new(&[("title", &title), ("summary", &summary)])
    }
}