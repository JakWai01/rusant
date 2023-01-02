pub mod template;

use crate::{rusant_contact_item::ContactItem, rusant_call_pane::CallPane};

use self::template::ContactListTemplate;

use gio::{ListStore, subclass::prelude::{ObjectSubclassExt, ObjectSubclassIsExt}};
use glib::{wrapper, StaticType, ObjectExt, clone};
use gtk::{Accessible, Box, Buildable, ConstraintTarget, Orientable, Widget, SingleSelection, traits::{WidgetExt, ButtonExt}};

wrapper! {
    pub struct ContactList(ObjectSubclass<ContactListTemplate>)
        @extends Widget, Box,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl Default for ContactList {
    fn default() -> Self {
        Self::new()
    }
}

impl ContactList {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn set_model(&self, model: Vec<ContactItem>, call_pane: &CallPane) {
        let template = ContactListTemplate::from_instance(self);
        let list_store_model = ListStore::new(ContactItem::static_type());

        for element in model {
            list_store_model.append(&element);
        }

        let selection_model = SingleSelection::new(Some(&list_store_model));

        template.list_box.bind_model(Some(&selection_model), clone!(@strong call_pane => move |x| {
            let name: String = x.property("name");
            
            let contact_item = ContactItem::new(&name);
            contact_item.handle_call_click(&call_pane);
            contact_item.handle_video_call_click(&call_pane);
            contact_item.avatar().set_text(Some(&name));
            contact_item.label().set_label(&name);

            let result = contact_item.ancestor(Widget::static_type());

            result.unwrap()
        }));  
    }

    // pub fn contact_item(&self) -> ContactItem {
    //     self.imp().contact_item.get()
    // }
}
