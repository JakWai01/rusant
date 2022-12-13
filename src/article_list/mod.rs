pub mod template;

use self::template::ArticleListTemplate;

use glib::{wrapper, Object};
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget};

wrapper! {
    pub struct ArticleList(ObjectSubclass<ArticleListTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for ArticleList {
    fn default() -> Self {
        Self::new()
    }
}

impl ArticleList {
    pub fn new() -> Self {
        Object::new(&[])
    }
}