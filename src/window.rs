use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::App;

mod imp {
    use crate::theme_parser::{serialise_hell, save_theme_settings};

    use super::*;

    #[derive(Debug)]
    pub struct Window {}

    impl Default for Window {
        fn default() -> Self {
            Self {}
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();

            let _box = gtk::Box::new(gtk::Orientation::Vertical, 0);

            let titlebar = adw::HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("rtheme hell", ""))
                .build();

            let button = gtk::Button::builder()
                .label("unleash demons")
                .css_classes(vec!["pill".into(), "suggested-action".into()])
                .build();

            let page = adw::StatusPage::builder()
                .title("rtheme hell")
                .description("click the button to create a random theme")
                .icon_name("applications-graphics-symbolic")
                .child(&button)
                .build();

            _box.append(&titlebar);
            _box.append(&page);

            button.connect_clicked(|_| {
                serialise_hell();
                save_theme_settings();
            });

            self.obj().set_content(Some(&_box));
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl Window {
    pub fn new(app: &App) -> Self {
        glib::Object::new(&[("application", app)])
    }
}
