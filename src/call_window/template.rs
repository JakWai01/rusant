use super::CallWindow;
use super::*;

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
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow};
use std::thread;
use gtk_macros::get_widget;
use libadwaita::prelude::ApplicationWindowExt;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/call-window.ui")]
pub struct CallWindowTemplate {
    #[template_child]
    pub grid: TemplateChild<FlowBox>,
}

#[object_subclass]
impl ObjectSubclass for CallWindowTemplate {
    const NAME: &'static str = "CallWindow";

    type Type = CallWindow;
    type ParentType = ApplicationWindow;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CallWindowTemplate {
    fn constructed(&self) {
        self.parent_constructed();

        let pipeline = gst::Pipeline::default();

        let src = gst::ElementFactory::make("v4l2src").build().unwrap();

        src.set_property("device", "/dev/video0");

        let caps = gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        let filter = gst::ElementFactory::make("capsfilter").build().unwrap();
        filter.set_property("caps", &caps);

        let convert = gst::ElementFactory::make("videoconvert").build().unwrap();

        let sink = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .unwrap();

        let paintable = sink.property::<gdk::Paintable>("paintable");
        pipeline
            .add_many(&[&src, &filter, &convert, &sink])
            .unwrap();

        gst::Element::link_many(&[&src, &filter, &convert, &sink]).unwrap();

        let picture = gtk::Picture::new();
        picture.set_paintable(Some(&paintable));
        // picture.set_property("keep-aspect-ratio", true);
        picture.set_keep_aspect_ratio(true);
        // picture.add_css_class("camera");

        let pipeline_test = gst::Pipeline::default();

        let src_test = gst::ElementFactory::make("videotestsrc").build().unwrap();

        // src_test.set_property("device", "/dev/video4");

        let caps_test =
            gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        let filter_test = gst::ElementFactory::make("capsfilter").build().unwrap();
        filter_test.set_property("caps", &caps_test);

        let convert_test = gst::ElementFactory::make("videoconvert").build().unwrap();

        let sink_test = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .unwrap();

        let paintable_test = sink_test.property::<gdk::Paintable>("paintable");
        pipeline_test
            .add_many(&[&src_test, &filter_test, &convert_test, &sink_test])
            .unwrap();

        gst::Element::link_many(&[&src_test, &filter_test, &convert_test, &sink_test]).unwrap();

        let picture_test = gtk::Picture::new();
        picture_test.set_paintable(Some(&paintable_test));
        picture_test.set_keep_aspect_ratio(true);


        let pipeline_demo= gst::Pipeline::default();

        let src_demo= gst::ElementFactory::make("videotestsrc").build().unwrap();

        // src_test.set_property("device", "/dev/video4");

        let caps_demo=
            gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        let filter_demo= gst::ElementFactory::make("capsfilter").build().unwrap();
        filter_demo.set_property("caps", &caps_demo);

        let convert_demo= gst::ElementFactory::make("videoconvert").build().unwrap();

        let sink_demo= gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .unwrap();

        let paintable_demo= sink_demo.property::<gdk::Paintable>("paintable");
        pipeline_demo
            .add_many(&[&src_demo, &filter_demo, &convert_demo, &sink_demo])
            .unwrap();

        gst::Element::link_many(&[&src_demo, &filter_demo, &convert_demo, &sink_demo]).unwrap();

        let picture_demo= gtk::Picture::new();
        picture_demo.set_paintable(Some(&paintable_demo));
        picture_demo.set_keep_aspect_ratio(true);

        self.grid.insert(&picture, 0);
        self.grid.insert(&picture_test, 1);
        self.grid.insert(&picture_demo, 2);
        
        thread::spawn(move || {
            pipeline
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");
        });
        thread::spawn(move || {
            pipeline_test
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");
        });
        thread::spawn(move || {
            pipeline_demo
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");
        });
    }
}

impl WidgetImpl for CallWindowTemplate {}
impl WindowImpl for CallWindowTemplate {}
impl ApplicationWindowImpl for CallWindowTemplate {}
impl AdwApplicationWindowImpl for CallWindowTemplate {}