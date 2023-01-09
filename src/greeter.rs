use gio::{subclass::prelude::ObjectSubclassIsExt, traits::NetworkMonitorExt};
use glib::Cast;
use gtk::subclass::widget::WidgetClassSubclassExt;
use gtk::subclass::widget::CompositeTemplate;

mod imp {
    use gio::subclass::prelude::{ObjectSubclass, ObjectImpl, ObjectImplExt, ObjectSubclassExt};
    use glib::{subclass::InitializingObject, clone};
    use gtk::{CompositeTemplate, TemplateChild, subclass::widget::WidgetImpl, prelude::InitializingWidgetExt};
    use libadwaita::subclass::prelude::BinImpl;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/jakobwaibel/Rusant/rusant-greeter.ui")]
    pub struct Greeter {
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub login_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Greeter {
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

    impl ObjectImpl for Greeter {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for Greeter {}

    impl BinImpl for Greeter {}
}

glib::wrapper! {
    pub struct Greeter(ObjectSubclass<imp::Greeter>)
        @extends gtk::Widget, libadwaita::Bin, @implements gtk::Accessible;
}

impl Greeter {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn default_widget(&self) -> gtk::Widget {
        self.imp().login_button.get().upcast()
    }
}