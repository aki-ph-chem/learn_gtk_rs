extern crate gtk;
use gtk::prelude::*;
use gtk::glib;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/use_clone.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.set_application(Some(app));

    let label: gtk::Label = builder.object("label")
        .expect("Error: label");

    // '+'ボタン
    let button_plus: gtk::Button = builder.object("button_plus")
        .expect("Error: button_plus");
    button_plus.connect_clicked(glib::clone!(@weak label => move |_| {
        let value = label.text()
            .parse().unwrap_or(0) + 1;
        label.set_text(&value.to_string());
    }));

    // '-'ボタン
    let button_minus: gtk::Button = builder.object("button_minus")
        .expect("Error: button_minus");
    button_minus.connect_clicked(glib::clone!(@weak label => move |_| {
        let value = label.text()
            .parse().unwrap_or(0) - 1;
        label.set_text(&value.to_string());
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("grid.button"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
