extern crate gtk;
use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/gtk_2.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.set_application(Some(app));

    let x = 10; let y = 12;
    let res_str = format!("x + y = {}", x + y); 

    let label: gtk::Label = builder.object("label_1").expect("Error: label_1");
    label.set_text(&res_str);

    window.show_all();
}

fn main(){
    let application = gtk::Application::new(
        Some("fuga.piyo"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
