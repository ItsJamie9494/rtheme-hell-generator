use log::{debug, info};

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::config::{APP_ID, VERSION};
use crate::window::Window;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct App {
        pub window: OnceCell<WeakRef<Window>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for App {
        const NAME: &'static str = "App";
        type Type = super::App;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for App {}

    impl ApplicationImpl for App {
        fn activate(&self) {
            debug!("AdwApplication<App>::activate");
            self.parent_activate();
            let app = self.instance();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = Window::new(&*app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self) {
            debug!("AdwApplication<App>::startup");
            self.parent_startup();
            let app = self.instance();

            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for App {}
    impl AdwApplicationImpl for App {}
}

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl App {
    fn main_window(&self) -> Window {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| {
                app.main_window().close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about_dialog();
            })
            .build();
        self.add_action_entries([action_quit, action_about])
            .unwrap();
    }

    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
        self.set_accels_for_action("window.close", &["<Control>w"]);
    }

    fn show_about_dialog(&self) {
        let devs: [String; 1] = ["Jamie Murphy".into()];

        // TODO
        let dialog = gtk::AboutDialog::builder()
            // TODO
            .logo_icon_name(APP_ID)
            .license_type(gtk::License::Gpl30)
            .version(VERSION)
            .transient_for(&self.main_window())
            .modal(true)
            .authors(devs.clone().into())
            .artists(devs.into())
            .build();

        dialog.present();
    }

    pub fn run(&self) {
        info!("rtheme hell ({})", APP_ID);
        info!("Version: {}", VERSION);

        ApplicationExtManual::run(self);
    }
}

impl Default for App {
    fn default() -> Self {
        glib::Object::new::<Self>(&[
            ("application-id", &APP_ID),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
    }
}
