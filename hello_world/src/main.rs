extern crate gtk;
use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failled to initialize GTK.");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello World");

    let label = gtk::Label::new(Some("Hello, World"));
    window.add(&label);

    window.show_all();

    gtk::main();
}
