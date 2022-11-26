mod application;
mod config;
mod theme_parser;
mod window;

use application::App;
use gtk::glib;

fn main() {
    pretty_env_logger::init();

    glib::set_application_name("rtheme hell");

    let app = App::default();
    app.run();
}
