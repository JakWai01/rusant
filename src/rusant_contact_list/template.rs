use glib::clone;

use super::ContactList;

use crate::rusant_contact_item::ContactItem;

use libadwaita::{HeaderBar, WindowTitle};

use glib::{
    self, object_subclass,
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
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    Box, CompositeTemplate, ListBox, Button, traits::{ButtonExt, WidgetExt}, ActionBar, MenuButton
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-list.ui")]
pub struct ContactListTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub list_box: TemplateChild<ListBox>,

    #[template_child]
    pub selection_button: TemplateChild<Button>,

    #[template_child]
    pub action_bar: TemplateChild<ActionBar>,

    #[template_child]
    pub title: TemplateChild<WindowTitle>,

    #[template_child]
    pub select_cancel_button: TemplateChild<Button>,

    #[template_child]
    pub add_button: TemplateChild<Button>,

    #[template_child]
    pub menu: TemplateChild<MenuButton>,
}

#[object_subclass]
impl ObjectSubclass for ContactListTemplate {
    const NAME: &'static str = "ContactList";

    type Type = ContactList;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        ContactItem::ensure_type();

        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ContactListTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        self.selection_button.connect_clicked(clone!(@weak self as contact_list => move |_| {
            contact_list.action_bar.set_revealed(true);

            contact_list.add_button.set_visible(false);
            contact_list.title.set_title("0 selected");
            contact_list.selection_button.set_visible(false);
            contact_list.menu.set_visible(false);

            contact_list.select_cancel_button.set_visible(true);
        }));

        self.select_cancel_button.connect_clicked(clone!(@weak self as contact_list => move |_| {
            contact_list.action_bar.set_revealed(false);

            contact_list.add_button.set_visible(true);
            contact_list.title.set_title("Contacts");
            contact_list.selection_button.set_visible(true);
            contact_list.menu.set_visible(true);

            contact_list.select_cancel_button.set_visible(false);
        }));
    }
}

impl WidgetImpl for ContactListTemplate {}
impl BoxImpl for ContactListTemplate {}
