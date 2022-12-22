use super::CallWindow;

use glib::{self, ObjectExt};
use gst::prelude::*;
use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
};
use gtk::{gdk, ffi::{GTK_POS_BOTTOM, GTK_POS_RIGHT}, traits::{GridExt, WidgetExt}};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate, Grid, FlowBox
};
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow};
use std::thread;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/call-window.ui")]
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
        pipeline.add_many(&[&src, &filter, &convert, &sink]).unwrap();

        gst::Element::link_many(&[&src, &filter, &convert, &sink]).unwrap();
        
        let picture = gtk::Picture::new();
        picture.set_paintable(Some(&paintable));
        // picture.set_property("keep-aspect-ratio", true);
        picture.set_keep_aspect_ratio(true);
        // picture.add_css_class("camera");

        let pipeline_test = gst::Pipeline::default();
       
        let src_test = gst::ElementFactory::make("v4l2src").build().unwrap();
        
        src_test.set_property("device", "/dev/video4");

        let caps_test = gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);
        
        let filter_test = gst::ElementFactory::make("capsfilter").build().unwrap();
        filter_test.set_property("caps", &caps_test);

        let convert_test = gst::ElementFactory::make("videoconvert").build().unwrap();

        let sink_test = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .unwrap();

        let paintable_test = sink_test.property::<gdk::Paintable>("paintable");
        pipeline_test.add_many(&[&src_test, &filter_test, &convert_test, &sink_test]).unwrap();

        gst::Element::link_many(&[&src_test, &filter_test, &convert_test, &sink_test]).unwrap();
        
        let picture_test = gtk::Picture::new();
        picture_test.set_paintable(Some(&paintable_test));
        picture_test.set_keep_aspect_ratio(true);
        // picture_test.add_css_class("camera");
        // picture_test.set_property("keep-aspect-ratio", true);

        // This actually specifies the resolution of the camera image
        // It might me useful to reduce the image in the pipeline as much as possible in order to save bandwidth
        // self.grid.attach(&picture, 0, 0, 320, 180);
        // self.grid.attach_next_to(&picture_test, Some(&picture), gtk::PositionType::__Unknown(GTK_POS_RIGHT), 320, 180);
        self.grid.insert(&picture, 0);
        self.grid.insert(&picture_test, 1);

        thread::spawn(move || {
            pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state");
        });
        thread::spawn(move || {
            pipeline_test.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state");
        });
    }
}

impl WidgetImpl for CallWindowTemplate {}
impl WindowImpl for CallWindowTemplate {}
impl ApplicationWindowImpl for CallWindowTemplate {}
impl AdwApplicationWindowImpl for CallWindowTemplate {}