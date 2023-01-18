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

        self.login_button
            .connect_clicked(clone!(@weak self as this => move |_| {
                let app = gtk::Application::new(None, Default::default());
                app.connect_activate(move |app| {
                    let window = ApplicationWindow::new(app);
                    window.set_default_size(800, 500);
                    window.set_title(Some("Rusant"));

                    let context = WebContext::default().unwrap();
                    let webview = WebView::with_context(&context);
                    webview.load_uri("https://github.com/JakWai01/rusant");
                    window.set_child(Some(&webview));

                    let settings = WebViewExt::settings(&webview).unwrap();
                    settings.set_enable_developer_extras(true);

                    window.show();
                });

                app.connect_shutdown(move |_| {
                    info!("Window was closed. Successfully authenticated!");

                    /*
                     * This is the success case if the authentication worked
                     * Later, this handler should close the application window
                     */
                    this.obj().parent_window().switch_to_leaflet();
                });

                app.run();
            }));
    }
}

impl WidgetImpl for GreeterTemplate {}

impl BinImpl for GreeterTemplate {}
