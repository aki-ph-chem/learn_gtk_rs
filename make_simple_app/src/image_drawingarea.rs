extern crate gtk;
use gtk::prelude::*;
use gtk::glib;
use gtk::gdk_pixbuf::Pixbuf;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/drawing_area.ui");
    let builder = gtk::Builder::from_string(ui);

    //window
    let window: gtk::Window = builder.object("window")
        .expect("Error: window");
    window.set_application(Some(app));

    // button
    let quit_button: gtk::Button = builder.object("quit_button")
        .expect("Error: quit_button");
    quit_button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.close();
    }));

    let path_to_image = "cat/pet_cat_sit.png";
    // drawing area
    let drawing_area: gtk::DrawingArea = builder.object("drawing_area")
        .expect("Error: drawing_area");
    match  Pixbuf::from_file_at_scale(path_to_image, 450, 450, true){
        Ok(pix) => {
            println!("Ok");
            drawing_area.connect_draw(move |widget, cr| {
                let width = widget.allocated_width();
                let height = widget.allocated_height();
                Inhibit(false)
            });
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
        },
    };

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("image.view"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
