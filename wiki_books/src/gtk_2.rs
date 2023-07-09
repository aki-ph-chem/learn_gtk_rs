extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    gtk::init()?;

    let ui = include_str!("ui/gtk_2.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1").expect("Error: window_1");
    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let x = 10; let y = 12;
    let res_str = format!("x + y = {}", x + y); 

    let label: gtk::Label = builder.object("label_1").expect("Error: label_1");
    label.set_text(&res_str);

    window.show_all();
    gtk::main();

    Ok(())
}
