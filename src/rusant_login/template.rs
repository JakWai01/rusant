use glib::subclass::InitializingObject;
use gtk::{prelude::InitializingWidgetExt, traits::ButtonExt};
use libadwaita::subclass::prelude::BinImpl;

use super::*;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-login.ui")]
pub struct LoginTemplate {
    #[template_child]
    pub next_button: TemplateChild<gtk::Button>,

    #[template_child]
    pub main_stack: TemplateChild<gtk::Stack>,

    #[template_child]
    pub back_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for LoginTemplate {
    const NAME: &'static str = "Login";
    type Type = super::Login;
    type ParentType = libadwaita::Bin;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for LoginTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        self.next_button
            .connect_clicked(clone!(@weak self as this => move |_| {
                this.obj().parent_window().switch_to_leaflet();
            }));

        self.back_button
            .connect_clicked(clone!(@weak self as this => move |_| {
                this.obj().parent_window().switch_to_greeter_page();
            }));
    }
}

impl WidgetImpl for LoginTemplate {}

impl BinImpl for LoginTemplate {}