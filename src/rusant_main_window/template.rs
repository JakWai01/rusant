use std::fs::File;

use super::MainWindow;
use super::*;

use crate::rusant_call_pane::CallPane;
use crate::rusant_contact_item::ContactData;
use crate::rusant_contact_list::template::ContactListTemplate;
use crate::rusant_contact_list::ContactList;
use crate::rusant_greeter::Greeter;
use crate::{rusant_call_pane::template::CallPaneTemplate, rusant_contact_item::ContactItem};
use serde::{Deserialize, Serialize};

use gio::prelude::ListModelExtManual;
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

use gtk::subclass::window::WindowImplExt;
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate, Stack,
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
    pub greeter: TemplateChild<Greeter>,

    #[template_child]
    pub contact_list: TemplateChild<ContactList>,

    #[template_child]
    pub call_pane: TemplateChild<CallPane>,

    #[template_child]
    pub main_stack: TemplateChild<Stack>,
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
    /// Construct a new MainWindow
    fn constructed(&self) {
        self.parent_constructed();

        let call_pane = self.call_pane.get();
        let call_pane_template = CallPaneTemplate::from_instance(&call_pane);

        let contact_list = self.contact_list.get();
        let contact_list_template = ContactListTemplate::from_instance(&contact_list);

        // Define show-start-title-buttons property for call_pane
        self.leaflet.property_expression("folded").bind(
            &call_pane_template.header_bar.get(),
            "show-start-title-buttons",
            Widget::NONE,
        );

        // Define visible property for call_pane
        self.leaflet.property_expression("folded").bind(
            &call_pane_template.back_button.get(),
            "visible",
            Widget::NONE,
        );

        // Define show-end-title-buttons for contact_list
        self.leaflet.property_expression("folded").bind(
            &contact_list_template.header_bar.get(),
            "show-end-title-buttons",
            Widget::NONE,
        );

        // Define initial contacts
        // let contact_model = vec![
        //     ContactItem::new("Jakob"),
        //     ContactItem::new("Felicitas"),
        //     ContactItem::new("Daniel"),
        // ];
        let contact_model = self.obj().restore_data();

        // Define the model contained in the ContactList
        self.contact_list.set_model(contact_model, &call_pane);
    }
}

impl WindowImpl for MainWindowTemplate {
    fn close_request(&self) -> glib::signal::Inhibit {
        // Store contacts in a vector
        let binding = self.contact_list.contacts().snapshot();

        let backup_data: Vec<ContactData> = binding
            .iter()
            .filter_map(Cast::downcast_ref::<ContactItem>)
            .map(ContactItem::to_contact_data)
            .collect();

        // Save state to file
        let file = File::create("data.json").expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data).expect("Could not write data to json file");

        // Pass close request on to the parent
        self.parent_close_request()
    }
}

impl WidgetImpl for MainWindowTemplate {}
impl ApplicationWindowImpl for MainWindowTemplate {}
impl AdwApplicationWindowImpl for MainWindowTemplate {}
