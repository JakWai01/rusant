use super::CallWindow;
use super::*;
use crate::call_section::template::CallSectionTemplate;
use crate::call_section::CallSection;
use crate::contact_list::ContactList;
use glib::subclass::types::ObjectSubclassExt;
use libadwaita::prelude::GObjectPropertyExpressionExt;
use crate::contact_list::template::ContactListTemplate;

use glib::{self, ObjectExt};
use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
};
use gst::prelude::*;
use gtk::{
    ffi::{GTK_POS_BOTTOM, GTK_POS_RIGHT},
    gdk,
    traits::{GridExt, WidgetExt},
};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate, FlowBox, Grid,
};
use gtk_macros::get_widget;
use libadwaita::prelude::ApplicationWindowExt;
use libadwaita::Leaflet;
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow};
use std::thread;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/call-window.ui")]
pub struct CallWindowTemplate {
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,

    // #[template_child]
    // pub grid: TemplateChild<FlowBox>,

    #[template_child]
    pub contact_list: TemplateChild<ContactList>,

    #[template_child]
    pub call_section: TemplateChild<CallSection>,
}

#[object_subclass]
impl ObjectSubclass for CallWindowTemplate {
    const NAME: &'static str = "CallWindow";

    type Type = CallWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        ContactList::ensure_type();
        CallSection::ensure_type();

        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CallWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        let call_section = self.call_section.get();
        let call_section_template = CallSectionTemplate::from_instance(&call_section);

        let contact_list = self.contact_list.get();
        let contact_list_template = ContactListTemplate::from_instance(&contact_list);

        self.leaflet.property_expression("folded").bind(
            &call_section_template.header_bar.get(),
            "show-start-title-buttons",
            Widget::NONE,
        );

        self.leaflet.property_expression("folded").bind(
            &call_section_template.back_button.get(),
            "visible",
            Widget::NONE,
        );

        self.leaflet.property_expression("folded").bind(
            &contact_list_template.header_bar.get(),
            "show-end-title-buttons",
            Widget::NONE,
        );
    }
}

impl WidgetImpl for CallWindowTemplate {}
impl WindowImpl for CallWindowTemplate {}
impl ApplicationWindowImpl for CallWindowTemplate {}
impl AdwApplicationWindowImpl for CallWindowTemplate {}
