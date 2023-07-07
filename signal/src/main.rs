extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello Wrold");
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();

    Ok(())
}
