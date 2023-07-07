extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello World");

    let vbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let label = gtk::Label::new(Some("Hello, World"));
    vbox.pack_start(&label, false, false, 5);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let entry = gtk::Entry::new();
    hbox.pack_start(&entry, false, false, 5);

    let button = gtk::Button::with_label("Ok");
    hbox.pack_start(&button, false, false, 5);

    vbox.pack_start(&hbox, true, true, 5);

    window.add(&vbox);
    window.show_all();

    gtk::main();
    Ok(())
}
