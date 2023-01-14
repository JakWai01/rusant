use crate::{
    receiver,
    sender::{self, Sender},
};

use super::CallPane;

use std::thread;

use anyhow::Error;
use derive_more::{Display, Error};
use glib::{
    self, clone, object_subclass,
    subclass::{
        object::{ObjectImpl, ObjectImplExt},
        types::ObjectSubclass,
        InitializingObject,
    },
    ObjectExt,
};
use libadwaita::{HeaderBar, StatusPage};

use gst::{
    element_error, element_warning, prelude::GstBinExtManual, prelude::*, traits::ElementExt,
};

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
use log::info;

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
            info!("Button `camera_video` was clicked");

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
            info!("Button `audio_input_microphone was clicked");

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
                info!("Button `call_stop` was clicked");

                // Hide call and show placeholder
                this.placeholder.set_visible(true);
                this.action_bar.set_visible(false);
                this.call_box.set_visible(false);
            }));

        /*
         * This part does not necessarily need to be here.
         * It just has to be started once a call starts but this can be anywhere.
         */
        let sender = sender::VideoSenderPipeline::new("127.0.0.1", 3000);
        // sender.build();

        thread::spawn(move || {
            sender.send();
        });

        let receiver = receiver::VideoReceiverPipeline::new("127.0.0.1", 3000);
        let (pipeline, paintable) = receiver.build();

        let picture = gtk::Picture::new();
        picture.set_paintable(Some(&paintable));
        picture.set_keep_aspect_ratio(true);

        self.grid.insert(&picture, 0);

        thread::spawn(move || {
            pipeline
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");
        });

        let audio_sender = sender::AudioSenderPipeline::new("127.0.0.1", 3001);
        audio_sender.build();

        thread::spawn(move || audio_sender.send());

        let audio_receiver = receiver::AudioReceiverPipeline::new("127.0.0.1", 3001);

        let audio_pipeline = audio_receiver.build();

        thread::spawn(move || {
            audio_pipeline
                .set_state(gst::State::Playing)
                .expect("Unable to set the audio pipeline to the `Playing` state");
        });
    }
}

impl WidgetImpl for CallPaneTemplate {}
impl BoxImpl for CallPaneTemplate {}
