extern crate gtk;
use gtk::prelude::*;
use gtk::glib;

fn build_ui(app: &gtk::Application) { 
    let ui = include_str!("ui/image.ui");
    let builder = gtk::Builder::from_string(ui);

    // window
    let window: gtk::Window = builder.object("window")
        .expect("Error: window");
    window.set_application(Some(app));

    // button: Clickで終了
    let button: gtk::Button = builder.object("button")
        .expect("Error: button");
    button.connect_clicked( glib::clone!(@weak window => move |_| {
        window.close();
    }));

    // 画像を表示する
    let path_to_image = "pet_cat_sit.png"; // いらすとやにあった猫の絵
    let image: gtk::Image = builder.object("image")
        .expect("Error: image");
    image.set_file(Some(path_to_image));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("image.view"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
