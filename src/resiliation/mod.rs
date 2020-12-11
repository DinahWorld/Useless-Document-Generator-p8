use crate::cv;
extern crate gtk;
use cv::Adress;
use gtk::prelude::*;
pub mod generate_letter;
pub mod letter;

#[derive(Clone)]
pub struct Letter {
    internet_box: Vec<gtk::CheckButton>,
    suscribe_nb: gtk::Entry,
    tel: gtk::Entry,
    adress: Adress,
    email: gtk::Entry,
}

impl Letter {
    fn build(builder: gtk::Builder) -> Letter {
        return Letter {
            internet_box: vec![
                builder.get_object("sfr").unwrap(),
                builder.get_object("red").unwrap(),
                builder.get_object("sosh").unwrap(),
                builder.get_object("orange").unwrap(),
                builder.get_object("freebox").unwrap(),
            ],
            suscribe_nb: builder.get_object("number").unwrap(),
            tel: builder.get_object("tel").unwrap(),
            adress: Adress::build(builder.clone()),
            email: builder.get_object("email").unwrap(),
        };
    }
}
