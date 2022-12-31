use super::MainWindow;
use super::*;

use std::cell::Cell;

use crate::{rusant_call_pane::template::CallPaneTemplate, rusant_contact_item::ContactItem};
use crate::rusant_call_pane::CallPane;
use crate::rusant_contact_list::template::ContactListTemplate;
use crate::rusant_contact_list::ContactList;
use crate::utils::UiState;

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
    pub contact_list: TemplateChild<ContactList>,

    #[template_child]
    pub call_section: TemplateChild<CallPane>,
}

#[object_subclass]
impl ObjectSubclass for MainWindowTemplate {
    const NAME: &'static str = "MainWindow";

    type Type = MainWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        ContactList::ensure_type();
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

        let contact_model = vec![ContactItem::new("Jakob"), ContactItem::new("Felicitas"), ContactItem::new("Daniel")];
        self.contact_list.set_model(contact_model);
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl WindowImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
