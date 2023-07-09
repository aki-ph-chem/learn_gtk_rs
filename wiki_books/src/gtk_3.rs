// ただwindowを表示するだけ
extern crate gtk;
use gtk::prelude::*;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    gtk::init()?;

    let ui = include_str!("ui/gtk_3.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1").expect("Error: window_1");
    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    let about: gtk::MenuItem = builder.object("about")
        .expect("Error: about"); 
    let dialog: gtk::MessageDialog = builder.object("dialog")
        .expect("Error: dialog");

    about.connect_popup_menu(move |_| {
        gtk::main_quit();
        false
    });


    /*
    about.connect_clicked(move |_| {
        let message = "GTK+"
        dialog.set_text(Some(message));
        dialog.run();
        unsafe {
            dialog.destroy();
        }
    });
    */

    window.show_all();
    gtk::main();

    Ok(())
}
