use gio::subclass::prelude::{ObjectSubclass, ObjectImpl, ObjectImplExt, ObjectSubclassExt};
use glib::{subclass::InitializingObject, clone};
use gtk::{CompositeTemplate, TemplateChild, subclass::widget::WidgetImpl, prelude::InitializingWidgetExt, traits::ButtonExt};
use libadwaita::subclass::prelude::BinImpl;

use super::*;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-greeter.ui")]
pub struct GreeterTemplate {
    // #[template_child]
    // pub back_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub login_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub register_button: TemplateChild<gtk::Button>,
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

        self.login_button.connect_clicked(clone!(@weak self as this => move |_| {
            this.obj().parent_window().switch_to_login_page();
        }));

        self.register_button.connect_clicked(clone!(@weak self as this => move |_| {
            this.obj().parent_window().switch_to_register_page();
        }));
    }
}

impl WidgetImpl for GreeterTemplate {}

impl BinImpl for GreeterTemplate {}