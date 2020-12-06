use crate::window;
extern crate gtk;
use gtk::prelude::*;

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

pub fn menu() {
    // First we get the file content.
    // Then we call the Builder call.
    let glade_src = include_str!("../glade/menu.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Menu").unwrap();
    let cv: gtk::Button = builder.get_object("CV").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    cv.connect_clicked(clone!(window => move |_| {
        window.hide();
        window::cv::cv();
        window.show();
    }));

    window.show_all();

    gtk::main();
}
