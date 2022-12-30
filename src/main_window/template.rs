use super::MainWindow;
use super::*;

use crate::call_section::template::CallPaneTemplate;
use crate::call_section::CallPane;
use crate::contact_list::template::ContactPaneTemplate;
use crate::contact_list::ContactPane;

use glib::{
    self, object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        types::ObjectSubclassExt,
        InitializingObject,
    },
};

use gst::prelude::*;

use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate,
};

use libadwaita::{
    prelude::GObjectPropertyExpressionExt, subclass::prelude::AdwApplicationWindowImpl,
    ApplicationWindow, Leaflet,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-main-window.ui")]
pub struct MainWindowTemplate {
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,

    #[template_child]
    pub contact_list: TemplateChild<ContactPane>,

    #[template_child]
    pub call_section: TemplateChild<CallPane>,
}

#[object_subclass]
impl ObjectSubclass for MainWindowTemplate {
    const NAME: &'static str = "MainWindow";

    type Type = MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        ContactPane::ensure_type();
        CallPane::ensure_type();

        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        let call_section = self.call_section.get();
        let call_section_template = CallPaneTemplate::from_instance(&call_section);

        let contact_list = self.contact_list.get();
        let contact_list_template = ContactPaneTemplate::from_instance(&contact_list);

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

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
