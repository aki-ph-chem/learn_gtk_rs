extern crate gtk;
use gtk::prelude::*;
use gtk::glib;
use gtk::gdk_pixbuf::Pixbuf;

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

    // Help/About
    let help_about: gtk::MenuItem = builder.object("help_about")
        .expect("Error: help_about");
    // Aboutdialog
    let about_dialog: gtk::AboutDialog = builder.object("about_dialog")
        .expect("Error: aoubt_dialog");
    help_about.connect_activate(glib::clone!(@weak window, @weak about_dialog => move |_| {
        about_dialog.run();
        about_dialog.set_transient_for(Some(&window));
        about_dialog.hide();
    }));

    // File/Open
    let file_chose: gtk::MenuItem = builder.object("file_chose")
        .expect("Error: file_chose");
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

    // 画像を表示する領域
    let image: gtk::Image = builder.object("image")
        .expect("Error: image");

    // エラーダイアログ
    let error_dialog: gtk::MessageDialog = builder.object("error_dialog")
        .expect("Error: error_dialog");
    // エラーメッセージ用のラベル
    let error_label: gtk::Label = builder.object("error_label")
        .expect("Error: error_label");
    // エラーダイアログ中のボタン
    let error_button: gtk::Button = builder.object("error_button")
        .expect("Error: error_button");
    error_button.connect_clicked(glib::clone!(@weak error_dialog => move|_| {
        error_dialog.hide();
    }));

    // 選択ダイアログ中のOpenをクリックで画像を開く
    file_chose_dialog.connect_response(glib::clone!(@weak image, @weak window  => move |fc_dialog, response| {
        if response == gtk::ResponseType::Ok {
            if let Some(path_to_image) = fc_dialog.filename(){
                let pix_buf = Pixbuf::from_file(path_to_image);
                    match pix_buf {
                        Ok(p) => image.set_pixbuf(Some(&p)),
                        Err(err) => { 
                            eprintln!("Error: {:?}", err);
                            error_label.set_label(&err.to_string());
                            error_dialog.set_title("Error");
                            error_dialog.set_transient_for(Some(&window));
                            error_dialog.run();
                        },
                    };
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
