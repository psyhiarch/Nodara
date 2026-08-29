use {
    adw::subclass::{application_window::AdwApplicationWindowImpl, prelude::*},
    gtk::{
        CompositeTemplate,
        gio::Settings,
        glib::{self, subclass::InitializingObject},
    },
    std::cell::OnceCell,
};

// Object holding the state
#[derive(CompositeTemplate, Default, Debug)]
#[template(resource = "/org/gtk_rs/Diagrams/window.ui")]
pub struct Window {
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub grid: TemplateChild<panel::Grid>,
}

#[gtk::template_callbacks]
impl Window {
    // #[template_callback]
    // fn handle_button_clicked(&self, button: &CustomButton) {
    //     let number_increased = self.number.get() + 1;
    //     self.number.set(number_increased);
    //     button.set_label(&number_increased.to_string())
    // }

    #[template_callback]
    fn create_frame_cb(&self, _dock: &panel::Dock) -> Option<panel::Frame> {
        // Return None to use libpanel's default fallback frame creation behavior,
        // or return a custom panel::Frame if you want to customize it.
        None
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "Diagrams";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks()
        //Self::Type::bind_template_callbacks(klass);
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");
        // Allow to invoke other event handlers
        glib::Propagation::Proceed
    }
}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
