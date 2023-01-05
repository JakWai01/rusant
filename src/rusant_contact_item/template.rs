use crate::rusant_main_window::MainWindow;

use super::ContactItem;

use std::cell::Cell;

use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    }, ParamSpec, once_cell::sync::Lazy, ParamSpecString, ParamFlags, Value, ToValue, clone
};

use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, WidgetImpl},
        widget::{CompositeTemplate, WidgetImplExt},
    },
    Box, CompositeTemplate, TemplateChild, Label, Button, CheckButton, traits::ButtonExt, ffi::gtk_widget_get_next_sibling
};

use libadwaita::Avatar;
use libadwaita::subclass::prelude::WidgetClassSubclassExt;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-item.ui")]
pub struct ContactItemTemplate {
    name: Cell<String>,

    #[template_child]
    pub avatar: TemplateChild<Avatar>,

    #[template_child]
    pub label: TemplateChild<Label>,

    #[template_child]
    pub call: TemplateChild<Button>,

    #[template_child]
    pub video_call: TemplateChild<Button>,

    #[template_child]
    pub selection: TemplateChild<CheckButton>,
}

#[object_subclass]
impl ObjectSubclass for ContactItemTemplate {
    const NAME: &'static str = "ContactItem";

    type Type = ContactItem;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ContactItemTemplate {
    fn constructed(&self) {
        self.parent_constructed();
        
        let contact_name = self.name.take();
        self.avatar.set_text(Some(&contact_name));
        self.label.set_label(&contact_name);
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new("name", "name", "The name of the contact", Some(""), ParamFlags::READWRITE)
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name_string = value.get().expect("The value needs to be of type `String`.");
                self.name.replace(name_string);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => {
                let result = self.name.take();

                self.name.set(result.clone());
                result.to_value()
            }
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ContactItemTemplate {}
impl BoxImpl for ContactItemTemplate {}
