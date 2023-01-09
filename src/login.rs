use gtk::{self, glib, subclass::prelude::*, CompositeTemplate};

mod imp {
    use glib::{subclass::InitializingObject};
    use gtk::prelude::InitializingWidgetExt;
    use libadwaita::subclass::prelude::BinImpl;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/jakobwaibel/Rusant/rusant-login.ui")]
    pub struct Login {
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Login {
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

    impl ObjectImpl for Login {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for Login {}

    impl BinImpl for Login {}
}

glib::wrapper! {
    pub struct Login(ObjectSubclass<imp::Login>)
        @extends gtk::Widget, libadwaita::Bin, @implements gtk::Accessible;
}

impl Login {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}