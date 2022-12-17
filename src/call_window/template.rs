use super::CallWindow;

use glib::{self, clone, MainContext, Continue, PRIORITY_DEFAULT, ObjectExt};
use gst::prelude::*;
use curio::prelude::Request;
use glib::{
    object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    StaticTypeExt,
};
use gtk::{gdk, prelude::PaintableExt, Image, ffi::gtk_snapshot_to_paintable};
use gtk::{
    prelude::InitializingWidgetExt,
    subclass::{
        application_window::ApplicationWindowImpl,
        prelude::{TemplateChild, WidgetImpl, WindowImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    CompositeTemplate, Box, Grid, Picture, WidgetPaintable
};
use libadwaita::{subclass::prelude::AdwApplicationWindowImpl, ApplicationWindow};
use rss::Channel;
use std::thread;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/call-window.ui")]
pub struct CallWindowTemplate {
    // #[template_child]
    // pub gtk_box: TemplateChild<Box>,

    // #[template_child]
    // pub grid: TemplateChild<Grid>,

    #[template_child]
    pub picture: TemplateChild<Picture>,
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
        
        // TODO: Find out why video0 is busy
        src.set_property("device", "/dev/video4");

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
        
        self.picture.set_paintable(Some(&paintable));
        
        thread::spawn(move || {
            pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state");
        });
    }
}

impl WidgetImpl for CallWindowTemplate {}
impl WindowImpl for CallWindowTemplate {}
impl ApplicationWindowImpl for CallWindowTemplate {}
impl AdwApplicationWindowImpl for CallWindowTemplate {}