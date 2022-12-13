use glib::{
    object_subclass,
    once_cell::sync::Lazy,
    subclass::{object::ObjectImpl, types::ObjectSubclass},
    ParamFlags, ParamSpec, ParamSpecString, ToValue, Value,
};
use std::cell::Cell;

// Object holding the state
#[derive(Default)]
pub struct ArticleItemTemplate {
    title: Cell<String>,
    summary: Cell<String>,
}

#[object_subclass]
impl ObjectSubclass for ArticleItemTemplate {
    const NAME: &'static str = "ArticleItem";
    type Type = super::ArticleItem;
}

impl ObjectImpl for ArticleItemTemplate {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new("title", "title", "The title of the RSS article", Some(""), ParamFlags::READWRITE),
                ParamSpecString::new("summary", "summary", "The summary of the RSS article", Some(""), ParamFlags::READWRITE),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "title" => {
                let title_string = value.get().expect("The value needs to be of type `String`.");
                self.title.replace(title_string);
            }
            "summary" => {
                let summary_string = value.get().expect("The value needs to be of type `String`.");
                self.summary.replace(summary_string);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "title" => {
                let result = self.title.take();

                self.title.set(result.clone());
                result.to_value()
            }
            "summary" => {
                let result = self.summary.take();

                self.summary.set(result.clone());
                result.to_value()
            }
            _ => unimplemented!(),
        }
    }
}