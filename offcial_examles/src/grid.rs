extern crate gtk;
use gtk::prelude::*;

// ToDo
// 2. Ctrl + q もしくは qが押されたときにプログラムを終了するようにする

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/grid.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::ApplicationWindow = builder.object("window")
        .expect("Error: windwow");
    window.set_application(Some(app));

    let grid: gtk::Grid = builder.object("grid_1")
        .expect("Error: grid_1");

    // button 6をクリックしてボタンサイズを変更
    let button_6: gtk::Button = builder.object("button_6")
        .expect("Error: button_6");
    let grid_button_6 = grid.clone();
    button_6.connect_clicked(move |button| {
        let height = grid_button_6.cell_height(button);
        let height_new = if height == 2 {
            1
        } else {
            2
        };
        grid_button_6.set_cell_height(button, height_new);
    });


    // button 7をクリックしてボタンの位置を変更
    let button_7: gtk::Button = builder.object("button_7")
        .expect("Error: button_6");
    let grid_button_7 = grid.clone();
    button_7.connect_clicked(move |button| {
        let left_attach = grid_button_7.cell_left_attach(button);
        let left_attach_new = if left_attach == 2 {
            0
        } else {
            left_attach + 1
        };
        grid_button_7.set_cell_left_attach(button, left_attach_new);
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("grid.button"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
