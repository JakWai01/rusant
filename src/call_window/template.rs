use super::CallWindow;

use glib::{self, clone, MainContext, Continue, PRIORITY_DEFAULT, ObjectExt};

use curio::prelude::Request;
use glib::{
    object_subclass,
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
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate,
};
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow, Leaflet};
use rss::Channel;
use std::thread;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/call-window.ui")]
pub struct CallWindowTemplate {
}

#[object_subclass]
impl ObjectSubclass for CallWindowTemplate {
    const NAME: &'static str = "CallWindow";

    type Type = CallWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CallWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for CallWindowTemplate {}
impl WindowImpl for CallWindowTemplate {}
impl ApplicationWindowImpl for CallWindowTemplate {}
impl AdwApplicationWindowImpl for CallWindowTemplate {}