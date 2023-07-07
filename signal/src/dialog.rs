extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello World");
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(true)
    });

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    let entry = gtk::Entry::new();
    hbox.pack_start(&entry, true, true, 5);

    let button = gtk::Button::with_label("Say");

    let window_ = window.clone(); 
    button.connect_clicked(move |_| {
        let message = format!("Text: {}", entry.text());

        let dialog = gtk::MessageDialog::new(
            Some(&window_),
            gtk::DialogFlags::empty(),
            gtk::MessageType::Info,
            gtk::ButtonsType::Ok,
            &message);

        dialog.run();
        unsafe {
            dialog.destroy();
        }
    });

    hbox.pack_start(&button, false, false, 5);

    window.add(&hbox);
    window.show_all();
    gtk::main();

    Ok(())
}
