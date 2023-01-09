use glib::{Cast, clone};
use gtk::{self, glib, subclass::prelude::*, CompositeTemplate, traits::WidgetExt};
use gtk_macros::spawn;

use crate::rusant_main_window::MainWindow;

mod imp {
    use glib::{subclass::InitializingObject};
    use gtk::{prelude::InitializingWidgetExt, traits::ButtonExt};
    use libadwaita::subclass::prelude::BinImpl;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/jakobwaibel/Rusant/rusant-register.ui")]
    pub struct Register {
        #[template_child]
        pub next_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,

        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Register {
        const NAME: &'static str = "Register";
        type Type = super::Register;
        type ParentType = libadwaita::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Register {
        fn constructed(&self) {
            self.parent_constructed();

            self.next_button.connect_clicked(clone!(@weak self as this => move |_| {
                this.obj().parent_window().switch_to_leaflet();
            }));
            
            self.back_button.connect_clicked(clone!(@weak self as this => move |_| {
                this.obj().parent_window().switch_to_greeter_page();
            }));
        }
    }

    impl WidgetImpl for Register {}

    impl BinImpl for Register {}
}

glib::wrapper! {
    pub struct Register(ObjectSubclass<imp::Register>)
        @extends gtk::Widget, libadwaita::Bin, @implements gtk::Accessible;
}

impl Register {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    fn parent_window(&self) -> MainWindow {
        self.root()
            .and_then(|root| root.downcast().ok())
            .expect("Register needs to have a parent window")
    }

    pub fn default_widget(&self) -> gtk::Widget {
        self.imp().next_button.get().upcast()
    }
}