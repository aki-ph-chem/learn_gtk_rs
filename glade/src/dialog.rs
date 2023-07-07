extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gtk::init()?;

    let ui = include_str!("ui/dialog.ui");
    let  builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");

    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let button: gtk::Button = builder.object("button")
        .expect("Error: button");
    let entry: gtk::Entry = builder.object("entry")
        .expect("Error: entry");
    let dialog: gtk::MessageDialog = builder.object("dialog")
        .expect("Error: dialog");

    // ちょっとこのへんの挙動がおかしい
    button.connect_clicked(move |_| {
        let message = format!("Text: {},", entry.text());
        dialog.set_text(Some(&message));
        dialog.run();
        unsafe {
            dialog.destroy();
        }
    });

    window.show_all();
    gtk::main();

    Ok(())
}
