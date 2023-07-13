extern crate gtk;
use gtk::prelude::*;
use gtk::glib;
use chrono;

fn current_time() -> String {
    format!("{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
}

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/clock.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::ApplicationWindow = builder.object("window")
        .expect("Error: window");
    window.set_application(Some(app));

    let time = current_time();
    let label: gtk::Label = builder.object("label")
        .expect("Error: label");
    label.set_text(&time);

    window.show_all();

}

fn main() {
    let application = gtk::Application::new(Some("window.clock"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
