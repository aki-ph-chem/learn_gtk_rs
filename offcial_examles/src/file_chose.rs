extern crate gtk;
use gtk::prelude::*;
use gtk::glib;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/file_chose.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.set_application(Some(app));

    // TextView: テキストが表示される領域
    let text_view: gtk::TextView = builder.object("text_view")
        .expect("Error: text_view");

    // menubar 
    let file_chose: gtk::MenuItem = builder.object("chose_file")
        .expect("Error: chose_file");

    // File/Openのクリックでファイル選択ダイアログを表示
    let file_chose_dialog: gtk::FileChooserDialog = builder
        .object("chose_file_dialog").expect("Error: chose_file_dialog");
    file_chose.connect_activate(glib::clone!(@weak window, @weak file_chose_dialog => move |_| {

        file_chose_dialog.set_title("open file");

        file_chose_dialog.set_transient_for(Some(&window));
        file_chose_dialog.run();
        file_chose_dialog.hide();
    }));

    // open_button
    let open_button: gtk::Button = builder.object("open_button")
        .expect("Error: open_button"); 
    open_button.connect_clicked(glib::clone!(@weak file_chose_dialog => move |_| {
        file_chose_dialog.set_response_sensitive(gtk::ResponseType::Ok, true);
    }));

    // cancel_button
    let cancel_button: gtk::Button = builder.object("cancel_button")
        .expect("Error: cancel_button"); 
    cancel_button.connect_clicked(glib::clone!(@weak file_chose_dialog => move |_| {
        file_chose_dialog.set_response_sensitive(gtk::ResponseType::Cancel, true);
    }));

    // open_button, cancle_buttonがクリックされた時の挙動
    file_chose_dialog.connect_response(glib::clone!(@weak text_view => move |fc_dialog, response| {
        if response == gtk::ResponseType::Ok {
            let filename = fc_dialog.filename().expect("Couldn't get filename");
            let file = File::open(filename).expect("Couldn't open file");

            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            text_view
                .buffer()
                .expect("Couldn't get window")
                .set_text(&contents);
        }
        fc_dialog.close();
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("file_chose.button"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
