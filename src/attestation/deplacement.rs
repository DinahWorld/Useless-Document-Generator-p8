use crate::window;
use crate::attestation;

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
fn true_or_false(choice : &Vec<gtk::CheckButton>) -> usize{
    for i in 0..9 {
        if choice[i].get_active(){
            return i;
        }
    }
    return 0;
}

pub fn create_attestation(user: &Rc<RefCell<window::User>>) {
    let glade_src = include_str!("../glade/attestation_deplacement.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Attestation").unwrap();

    let hour: gtk::Entry = builder.get_object("hours").unwrap();
    let adress = window::Adress::build(builder.clone());

    let choice : Vec<gtk::CheckButton> = vec![builder.get_object("1").unwrap(),
    builder.get_object("2").unwrap(), builder.get_object("3").unwrap(), builder.get_object("4").unwrap(),
    builder.get_object("5").unwrap(), builder.get_object("6").unwrap(), builder.get_object("7").unwrap(),
    builder.get_object("8").unwrap(),builder.get_object("9").unwrap()];

    let validate: gtk::Button = builder.get_object("validate").unwrap();
  


    validate.connect_clicked(clone!(user,adress,hour => move |_| {
        let choice = true_or_false(&choice);
        attestation::generate_attestation::attestation(
            &user,
            &adress,
            &hour,
            choice + 1);
    }),);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
