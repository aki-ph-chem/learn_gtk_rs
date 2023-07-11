extern crate gtk;
use gtk::prelude::*;

// ToDo
// 1. button_6, button_7が押されたときのイベント処理
// 2. Ctrl + q もしくは qが押されたときにプログラムを終了するようにする

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/grid.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::ApplicationWindow = builder.object("window")
        .expect("Error: windwow");
    window.set_application(Some(app));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("grid.button"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
