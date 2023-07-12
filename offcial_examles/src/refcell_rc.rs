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

fn build_ui(application: &gtk::Application){
    let ui = include_str!("ui/refcell_rc.ui");
    let builder = gtk::Builder::from_string(ui);

    let p_1 = Rc::new(RefCell::new(Point::new()));
    let p_2 = Rc::new(RefCell::new(Point::new()));

    p_1.borrow_mut().state = true;

    // window
    let window: gtk::Window = builder.object("window_1") 
        .expect("Error: window_1");
    window.set_application(Some(application));

    // button 
    let button: gtk::Button = builder.object("button")
        .expect("Error: button");
    button.connect_clicked(
        move |_| {
            let mut p_1 = p_1.borrow_mut();
            let mut p_2 = p_2.borrow_mut();

            println!("Clicked: (p_1.state, p_1.x, p_1.y) = ({}, {}, {})",
            p_1.state, p_1.x, p_1.y );
            println!("Clicked: (p_2.state, p_2.x, p_2.y) = ({}, {}, {})",
            p_2.state, p_2.x, p_2.y );

            p_1.x += 1; p_1.y += 2;
            p_2.x -= 1; p_2.y -= 2;
        }
        );

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("refcell.rc"),Default::default());

    application.connect_activate(build_ui);
    application.run();
}
