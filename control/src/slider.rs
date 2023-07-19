use gtk;
use gtk::prelude::*;
use gtk::glib;
use std::cell::RefCell;
use std::rc::Rc;

struct Position {
    scale_1: f64,
    scale_2: f64,
}

impl Position {
    fn new() -> Position {
        Position{scale_1: 0.0 ,scale_2: 0.0}
    }

    fn sum(&self) -> f64 {
        self.scale_1 + self.scale_2
    }
}

fn build_ui(app: &gtk::Application) {
    let ui = include_str!("ui/slider.ui");
    let builder = gtk::Builder::from_string(ui);

    // window
    let window: gtk::ApplicationWindow = builder.object("window")
        .expect("Error: window");
    window.set_application(Some(app));

    // scalses
    let scale_1: gtk::Scale = builder.object("scale_1")
        .expect("Error: scale_1");
    let scale_2: gtk::Scale = builder.object("scale_2")
        .expect("Error: scale_1");

    // 現在のscaleの値を保持するためのオブジェクト
    let current_level = Rc::new(RefCell::new(Position::new()));
    // label
    let label: gtk::Label = builder.object("label")
        .expect("Error: label");
    label.set_label("x + y");

    // 二個のScaleを動かした時に、それぞれ値の和をlabelに表示
    scale_1.connect_value_changed(glib::clone!(@weak scale_1, @weak current_level, @weak label => move |_| {
        let level_scale_1 = scale_1.value();
        let mut current_scale_1 = current_level.borrow_mut();
        current_scale_1.scale_1 = level_scale_1;

        let res = format!("x + y = {}", current_scale_1.sum());
        label.set_label(&res);
    }));
    scale_2.connect_value_changed(glib::clone!(@weak scale_2, @weak label => move |_| {
        let level_scale_2 = scale_2.value();
        let mut current_scale_2 = current_level.borrow_mut();
        current_scale_2.scale_2 = level_scale_2;

        let res = format!("x + y = {}", current_scale_2.sum());
        label.set_label(&res);
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("slider.app"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
