pub mod template;

use self::template::FeedListTemplate;
use crate::feed_item::FeedItem;

use glib::{wrapper, Object, subclass::types::ObjectSubclassExt, ObjectExt, StaticType};
use gtk::{gio::ListStore, traits::WidgetExt, Accessible, Box, Buildable, ConstraintTarget, Orientable, SingleSelection, Widget};
use libadwaita::ActionRow;

wrapper! {
    pub struct FeedList(ObjectSubclass<FeedListTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for FeedList {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedList {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn set_model(&self, model: Vec<FeedItem>) {
        let template = FeedListTemplate::from_instance(self);
        let list_store_model = ListStore::new(FeedItem::static_type());

        for element in model {
            list_store_model.append(&element);
        }

        let selection_model = SingleSelection::new(Some(&list_store_model));

        template.list_box.bind_model(Some(&selection_model), |x| {
            let name: String = x.property("name");
            let action_row = ActionRow::builder().title(&name).build();
            let result = action_row.ancestor(Widget::static_type());

            result.unwrap()
        });
    }
}