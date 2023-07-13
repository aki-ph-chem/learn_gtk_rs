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

    // File/Quitをクリックでプログラムを終了
    let quit: gtk::MenuItem = builder.object("quit")
        .expect("Error: quit");
    quit.connect_activate(move |_|{
        gtk::main_quit();
    });
    // Ctrl + qで終了
    let accel_group = gtk::AccelGroup::new();
    window.add_accel_group(&accel_group);
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    quit.add_accelerator("activate", &accel_group, 
                         key, modifier, gtk::AccelFlags::VISIBLE);


    // AbouからAboutDialogが生成される
    let about: gtk::MenuItem = builder.object("about")
        .expect("Error: about"); 
    let about_dialog: gtk::AboutDialog = builder.object("about_dialog")
        .expect("Error: about_dialog");

    let window_ = window.clone();
    about.connect_activate(move |_|{
        about_dialog.set_title("About");

        about_dialog.set_authors(&["Aki"]);
        about_dialog.set_transient_for(Some(&window_));
        about_dialog.run();
        about_dialog.hide();
    });

    window.show_all();
    gtk::main();

    Ok(())
}
