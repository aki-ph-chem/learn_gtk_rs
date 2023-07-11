extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main()-> Result<(), Box<dyn Error>> {
    gtk::init()?;
    let ui = include_str!("ui/button_inc.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(true)
    });

    let label: gtk::Label = builder.object("label")
        .expect("Error: label");

    let button_inc: gtk::Button = builder.object("button_inc")
        .expect("Error: button_inc");
    let label_ = label.clone();
    button_inc.connect_clicked(move |_| {
        let value = label_.text()
            .parse()
            .unwrap_or(0) + 1;
        label_.set_text(&value.to_string());
    });

    let button_dec: gtk::Button = builder.object("button_dec")
        .expect("Error: button_dec");
    button_dec.connect_clicked(move |_| {
        let value = label.text()
            .parse()
            .unwrap_or(0) - 1;
        label.set_text(&value.to_string());
    });

    window.show_all();
    gtk::main();

    Ok(())
}
