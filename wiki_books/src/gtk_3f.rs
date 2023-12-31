// ただwindowを表示するだけ
extern crate gtk;
use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/gtk_3.ui");
    let builder = gtk::Builder::from_string(ui);
    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");

    window.set_application(Some(app));

    // File/Quitをクリックでプログラムを終了
    let quit: gtk::MenuItem = builder.object("quit")
        .expect("Error: quit");
    let window_ = window.clone();
    quit.connect_activate(move |_|{
        window_.close();
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
        about_dialog.set_authors(&["Aki"]);
        about_dialog.set_title("About");
        about_dialog.set_transient_for(Some(&window_));
        about_dialog.run();
        about_dialog.hide();
    });

    window.show_all();
}

fn main(){

    let application = gtk::Application::new(
        Some("fuga.piyo"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
