extern crate gtk;
use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/gtk_1/gtk_1.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.set_application(Some(app));

    // Ctrl + qで終了
    let accel_group = gtk::AccelGroup::new();
    window.add_accel_group(&accel_group);
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");

    window.show_all();
}

fn main(){

    let application = gtk::Application::new(
        Some("hoge.fuga"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
