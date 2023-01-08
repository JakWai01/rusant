use super::ContactItem;

use std::cell::Cell;
use std::cell::RefCell;

use glib::ParamSpecBoolean;
use glib::{
    object_subclass,
    once_cell::sync::Lazy,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    Binding, ParamFlags, ParamSpec, ParamSpecString, ToValue, Value,
};

use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, WidgetImpl},
        widget::CompositeTemplate,
    },
    Box, Button, CheckButton, CompositeTemplate, Label, TemplateChild,
};

use libadwaita::subclass::prelude::WidgetClassSubclassExt;
use libadwaita::Avatar;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-contact-item.ui")]
pub struct ContactItemTemplate {
    name: Cell<String>,
    active: Cell<bool>,

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

    pub bindings: RefCell<Vec<Binding>>,
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
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new(
                    "name",
                    "name",
                    "The name of the contact",
                    Some(""),
                    ParamFlags::READWRITE,
                ),
                ParamSpecBoolean::new(
                    "active",
                    "active",
                    "If the contact is currently marked",
                    false,
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "name" => {
                let name_string = value
                    .get()
                    .expect("The value needs to be of type `String`.");
                self.name.replace(name_string);
            }
            "active" => {
                let active = value.get().expect("The value needs to be of type `bool`.");
                self.active.replace(active);
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
            "active" => {
                let result = self.active.take();

                self.active.set(result.clone());
                result.to_value()
            }
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ContactItemTemplate {}
impl BoxImpl for ContactItemTemplate {}
