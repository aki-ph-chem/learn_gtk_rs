extern crate gtk;
use gtk::prelude::*;
use gtk::glib;

fn build_ui(app: &gtk::Application) { 
    let ui = include_str!("ui/image.ui");
    let builder = gtk::Builder::from_string(ui);

    // window
    let window: gtk::Window = builder.object("window")
        .expect("Error: window");
    window.set_application(Some(app));

    // ショートカットキー
    let accel_group = gtk::AccelGroup::new(); 
    window.add_accel_group(&accel_group);

    // button: Clickで終了
    let button: gtk::Button = builder.object("button")
        .expect("Error: button");
    button.connect_clicked( glib::clone!(@weak window => move |_| {
        window.close();
    }));

    // File/Quit: クリックで終了
    let file_quit: gtk::MenuItem = builder.object("file_quit")
        .expect("Error: file_quit");
    file_quit.connect_activate(glib::clone!(@weak window => move |_| {
        window.close();
    }));
    // Ctrl + qで終了
    let (key, modifier) = gtk::accelerator_parse("<Primary>Q");
    file_quit.add_accelerator("activate", &accel_group, 
                              key, modifier, gtk::AccelFlags::VISIBLE);

    // File/Open
    let file_chose: gtk::MenuItem = builder.object("file_chose")
        .expect("Error: open");
    // file_chose_dialog
    let file_chose_dialog: gtk::FileChooserDialog = 
        builder.object("file_chose_dialog").expect("Error: file_chose_dialog");

    // File/Openのクリックでファイル選択ダイアログを表示
    file_chose.connect_activate(glib::clone!(@weak window, @weak file_chose_dialog => move |_| {
        file_chose_dialog.set_title("open file");
        file_chose_dialog.set_transient_for(Some(&window));
        file_chose_dialog.run();
        file_chose_dialog.hide();
    }));
    // Ctrl + oでファイル選択ダイアログを表示
    let (key, modifier) = gtk::accelerator_parse("<Primary>O"); 
    file_chose.add_accelerator("activate", &accel_group,
                               key, modifier, gtk::AccelFlags::VISIBLE);

    // 画像を表示する
    let path_to_image = "cat/pet_cat_sit.png"; // いらすとやにあった猫の絵
    let image: gtk::Image = builder.object("image")
        .expect("Error: image");
    image.set_file(Some(path_to_image));

    // 選択ダイアログ中のOpenをクリックで画像を開く
    file_chose_dialog.connect_response(glib::clone!(@weak image => move |fc_dialog, response| {
        if response == gtk::ResponseType::Ok {
            if let Some(path_to_image) = fc_dialog.filename()
                .expect("Couldn't get filename").to_str() {
                image.set_file(Some(path_to_image));
            }
        }
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("image.view"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
