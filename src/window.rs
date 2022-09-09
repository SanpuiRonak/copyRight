use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/github/sanpuironak/copyright/window.ui")]
    pub struct CopyrightWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CopyrightWindow {
        const NAME: &'static str = "CopyrightWindow";
        type Type = super::CopyrightWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CopyrightWindow {}
    impl WidgetImpl for CopyrightWindow {}
    impl WindowImpl for CopyrightWindow {}
    impl ApplicationWindowImpl for CopyrightWindow {}
}

glib::wrapper! {
    pub struct CopyrightWindow(ObjectSubclass<imp::CopyrightWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl CopyrightWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create CopyrightWindow")
    }
}
