// ただwindowを表示するだけ
extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    gtk::init()?;

    let ui = include_str!("ui/gtk_1/gtk_1_2.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1").expect("Error: window_1");
    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();

    Ok(())
}
