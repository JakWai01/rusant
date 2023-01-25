use gio::{
    subclass::prelude::{ObjectImpl, ObjectImplExt, ObjectSubclass, ObjectSubclassExt},
    traits::ApplicationExt,
};
use glib::{clone, subclass::InitializingObject, ToVariant};
use gtk::prelude::*;
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::widget::WidgetImpl,
    traits::{ButtonExt, GtkWindowExt},
    ApplicationWindow, CompositeTemplate, TemplateChild, Window,
};
use libadwaita::subclass::prelude::BinImpl;
use log::info;
use webkit2gtk::{prelude::*, WebContext, WebView};

use super::*;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-greeter.ui")]
pub struct GreeterTemplate {
    #[template_child]
    pub login_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for GreeterTemplate {
    const NAME: &'static str = "Greeter";
    type Type = super::Greeter;
    type ParentType = libadwaita::Bin;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        klass.set_accessible_role(gtk::AccessibleRole::Group);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GreeterTemplate {
    fn constructed(&self) {
        self.parent_constructed();
    }
}

impl WidgetImpl for GreeterTemplate {}

impl BinImpl for GreeterTemplate {}
