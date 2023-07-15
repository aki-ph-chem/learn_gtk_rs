extern crate gtk;
use gtk::prelude::*;
use gtk::glib;

// gtk::Builderを使わない場合ではコマンドライン引数から画像へのpathを受け取る処理は簡単に書ける
fn main() {
    let args:Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: invalid args");
        std::process::exit(1);
    }

    let path_to_image = &args[1];

    gtk::init().unwrap();
    let ui = include_str!("ui/image.ui");
    let builder = gtk::Builder::from_string(ui);

    // window
    let window: gtk::Window = builder.object("window")
        .expect("Error: window");
    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });

    // button: Clickで終了
    let button: gtk::Button = builder.object("button")
        .expect("Error: button");
    button.connect_clicked( glib::clone!(@weak window => move |_| {
        window.close();
    }));

    // 画像を表示する
    let image: gtk::Image = builder.object("image")
        .expect("Error: image");
    image.set_file(Some(path_to_image));

    window.show_all();
    gtk::main();
}
