extern crate gtk;
use gtk::glib;
use gtk::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Point {
    state: bool,
    x: i32,
    y: i32,
} 

impl Point {
    fn new() -> Self{
        Point {
            state: false,
            x: 0,
            y: 0,
        }
    }
}

fn build_ui(app: &gtk::Application){
    let ui = include_str!("ui/refcell_rc.ui");
    let builder = gtk::Builder::from_string(ui);

    let window: gtk::Window = builder.object("window_1")
        .expect("Error: window_1");
    window.set_application(Some(app));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("refcell.rc"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
