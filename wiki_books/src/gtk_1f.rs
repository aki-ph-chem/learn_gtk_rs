extern crate gtk;
use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/gtk_1/gtk_1_3.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.set_application(Some(app));

    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
}

fn main(){

    let application = gtk::Application::new(
        Some("hoge.fuga"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
