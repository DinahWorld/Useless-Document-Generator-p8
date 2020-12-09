use crate::window;
extern crate gtk;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

pub fn menu(new_user: window::User) {
    let glade_src = include_str!("../glade/menu.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Menu").unwrap();
    let cv: gtk::Button = builder.get_object("CV").unwrap();
    let user: Rc<RefCell<window::User>> = Rc::new(RefCell::new(new_user));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    cv.connect_clicked(clone!(user,window => move |_| {
        window.hide();
        window::cv::create_cv(&user);
        window.show();
    }));

    window.show_all();

    gtk::main();
}
