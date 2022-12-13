pub mod template;

use self::template::ArticleListTemplate;
use crate::article_item::ArticleItem;

use glib::{wrapper, Object, subclass::types::ObjectSubclassExt, ObjectExt, StaticType};
use gtk::{Align, Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget, SingleSelection, Label, gio::ListStore, traits::WidgetExt};
use libadwaita::{ExpanderRow, traits::ExpanderRowExt};

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

    pub fn set_model(&self, model: Vec<ArticleItem>) {
        let template = ArticleListTemplate::from_instance(self);
        let list_store_model = ListStore::new(ArticleItem::static_type());

        for element in model {
            list_store_model.append(&element);
        }

        let selection_model = SingleSelection::new(Some(&list_store_model));

        template.list_box.bind_model(Some(&selection_model), |x| {
            let title: String = x.property("title");
            let summary: String = x.property("summary");

            let expander_content = &Label::builder()
                .label(&summary)
                .halign(Align::Start)
                .margin_top(24)
                .margin_bottom(24)
                .margin_start(12)
                .margin_end(12)
                .build();
            let expander_content_widget = expander_content.ancestor(Widget::static_type());

            let expander_row = ExpanderRow::builder().title(&title).build();
            let expander_row_widget = expander_row.ancestor(Widget::static_type());

            expander_row.add_row(&expander_content_widget.unwrap());
            expander_row_widget.unwrap()
        });
    }
}