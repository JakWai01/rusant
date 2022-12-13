use super::ArticleList;

use glib::{
    object_subclass,
    once_cell::sync::Lazy,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    ParamFlags, ParamSpec, ParamSpecBoolean, ToValue, Value,
};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    traits::WidgetExt,
    Box, Button, CompositeTemplate, ListBox
};
use libadwaita::HeaderBar;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/article-list.ui")]
pub struct ArticleListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub list_box: TemplateChild<ListBox>,

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
     fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecBoolean::new(
                    "show-start-title-buttons",
                    "show-start-title-buttons",
                    "Shows the title buttons in the header bar",
                    false,
                    ParamFlags::READWRITE,
                ),
                ParamSpecBoolean::new(
                    "show-back-button",
                    "show-back-button",
                    "Shows the back button in the header bar",
                    false,
                    ParamFlags::READWRITE,
                ),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "show-start-title-buttons" => {
                let bool_value = value.get().expect("The value needs to be of type `bool`.");
                self.header_bar.set_show_start_title_buttons(bool_value);
            }
            "show-back-button" => {
                let bool_value = value.get().expect("The value needs to be of type `bool`.");
                self.back_button.set_visible(bool_value);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "show-start-title-buttons" => self.header_bar.shows_start_title_buttons().to_value(),
            "show-back-button" => self.back_button.is_visible().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for ArticleListTemplate {}
impl BoxImpl for ArticleListTemplate {}