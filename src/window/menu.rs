use crate::window;
extern crate gtk;
use gtk::prelude::*;

fn close(window: &gtk::Window){

    window.hide();
}
fn show(window: &gtk::Window){
    window.show_all();
}

pub fn menu(){
    // First we get the file content.
    // Then we call the Builder call.
    let glade_src = include_str!("../glade/menu.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window:  gtk::Window = builder.get_object("Menu").unwrap();
    let cv: gtk::Button = builder.get_object("CV").unwrap();
    let cancel: gtk::Button = builder.get_object("cancel").unwrap();
    
    let window_clone = window.clone();
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    cv.connect_clicked(move |_| {
        window.hide();
        window::cv::cv()
    });
  

    window_clone.show_all();

    gtk::main();
}