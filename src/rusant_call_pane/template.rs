use super::CallPane;

use std::thread;

use libadwaita::{HeaderBar, StatusPage};

use glib::{
    self, clone, object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    ObjectExt,
};

use gst::{prelude::GstBinExtManual, traits::ElementExt};

use gtk::{
    gdk,
    prelude::InitializingWidgetExt,
    subclass::{
        prelude::{BoxImpl, TemplateChild, WidgetImpl},
        widget::{CompositeTemplate, WidgetClassSubclassExt},
    },
    traits::{ButtonExt, WidgetExt},
    ActionBar, Box, Button, CompositeTemplate, FlowBox,
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/jakobwaibel/Rusant/rusant-call-pane.ui")]
pub struct CallPaneTemplate {
    #[template_child]
    pub header_bar: TemplateChild<HeaderBar>,

    #[template_child]
    pub back_button: TemplateChild<Button>,

    #[template_child]
    pub grid: TemplateChild<FlowBox>,

    #[template_child]
    pub placeholder: TemplateChild<StatusPage>,

    #[template_child]
    pub call_box: TemplateChild<Box>,

    #[template_child]
    pub action_bar: TemplateChild<ActionBar>,

    #[template_child]
    pub camera_video: TemplateChild<Button>,

    #[template_child]
    pub audio_input_microphone: TemplateChild<Button>,

    #[template_child]
    pub call_stop: TemplateChild<Button>,
}

#[object_subclass]
impl ObjectSubclass for CallPaneTemplate {
    const NAME: &'static str = "CallPane";

    type Type = CallPane;
    type ParentType = Box;

    fn class_init(my_class: &mut Self::Class) {
        Self::bind_template(my_class);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CallPaneTemplate {
    /// Construct a new CallPane
    fn constructed(&self) {
        self.parent_constructed();

        // Handle click on camera_video button
        self.camera_video.connect_clicked(move |button| {
            let css_class = "suggested-action";

            // Check if the button currently has the `suggested-action` css class
            if button.has_css_class(css_class) {
                button.remove_css_class(css_class);
            } else {
                button.add_css_class(css_class);
            }
        });

        // Handle click on audio_input_microphone button
        self.audio_input_microphone.connect_clicked(move |button| {
            let css_class = "suggested-action";

            // Check if button currently has the `suggested-action` css class
            if button.has_css_class(css_class) {
                button.remove_css_class(css_class);
            } else {
                button.add_css_class(css_class);
            }
        });

        // Handle click on call_stop button
        self.call_stop
            .connect_clicked(clone!(@weak self as this => move |_| {

                // Hide call and show placeholder
                this.placeholder.set_visible(true);
                this.action_bar.set_visible(false);
                this.call_box.set_visible(false);
            }));

        // Initialize testing gstreamer pipeline
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
        picture.set_keep_aspect_ratio(true);

        let pipeline_test = gst::Pipeline::default();

        let src_test = gst::ElementFactory::make("videotestsrc").build().unwrap();

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

        let pipeline_demo = gst::Pipeline::default();

        let src_demo = gst::ElementFactory::make("videotestsrc").build().unwrap();

        let caps_demo =
            gst::Caps::new_simple("video/x-raw", &[("width", &640i32), ("height", &480i32)]);

        let filter_demo = gst::ElementFactory::make("capsfilter").build().unwrap();
        filter_demo.set_property("caps", &caps_demo);

        let convert_demo = gst::ElementFactory::make("videoconvert").build().unwrap();

        let sink_demo = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .unwrap();

        let paintable_demo = sink_demo.property::<gdk::Paintable>("paintable");
        pipeline_demo
            .add_many(&[&src_demo, &filter_demo, &convert_demo, &sink_demo])
            .unwrap();

        gst::Element::link_many(&[&src_demo, &filter_demo, &convert_demo, &sink_demo]).unwrap();

        let picture_demo = gtk::Picture::new();
        picture_demo.set_paintable(Some(&paintable_demo));
        picture_demo.set_keep_aspect_ratio(true);

        self.grid.insert(&picture, 0);
        self.grid.insert(&picture_test, 1);
        // self.grid.insert(&picture_demo, 2);

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

impl WidgetImpl for CallPaneTemplate {}
impl BoxImpl for CallPaneTemplate {}
