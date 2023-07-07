extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let ui = include_str!("ui/hello.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::Window = builder.object("window_1").expect("Error: window_1");
    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button: gtk::Button = builder.object("button").expect("Error: button"); 
    let entry: gtk::Entry = builder.object("entry").expect("Error: entry");
    button.connect_clicked(move |_| {
        println!("Text: {}", entry.text());
    });

    window.show_all();
    gtk::main();

    Ok(())
}
