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

    let args:Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: invalid args");
        std::process::exit(1);
    }

    // 引数をLabelに表示する
    let res_str = format!("args[1] = {}", args[1]); 
    let label: gtk::Label = builder.object("label_1").expect("Error: label_1");
    label.set_text(&res_str);

    window.show_all();
    gtk::main();

    Ok(())
}
