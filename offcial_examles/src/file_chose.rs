extern crate gtk;
use gtk::prelude::*;
use gtk::glib;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/file_chose.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.set_application(Some(app));

    // menubar 
    let file_chose: gtk::MenuItem = builder.object("chose_file")
        .expect("Error: chose_file");

    // File/Openのクリックでファイル選択ダイアログを表示
    let file_chose_dialog: gtk::FileChooserDialog = builder
        .object("chose_file_dialog").expect("Error: chose_file_dialog");
    file_chose.connect_activate(glib::clone!(@weak window => move |_| {
        file_chose_dialog.set_title("open file");

        file_chose_dialog.set_transient_for(Some(&window));
        file_chose_dialog.run();
        file_chose_dialog.hide();
    }));

    // open_button
    let open_button: gtk::Button = builder.object("open_button")
        .expect("Error: open_button"); 
    // cancel_button
    let cancel_button: gtk::Button = builder.object("cancel_button")
        .expect("Error: cancel_button"); 

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("file_chose.button"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
