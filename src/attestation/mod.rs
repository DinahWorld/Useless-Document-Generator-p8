use crate::cv;
extern crate gtk;
use gtk::prelude::*;
pub mod deplacement;
pub mod generate_attestation;
use cv::Adress;

#[derive(Clone)]
pub struct Choice {
    adress: Adress,
    hour: gtk::Entry,
    choice: Vec<gtk::CheckButton>,
}

impl Choice {
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    fn build(builder: gtk::Builder) -> Choice {
        return Choice {
            adress: Adress::build(builder.clone()),
            hour: builder.get_object("hours").unwrap(),
            choice: vec![
                builder.get_object("1").unwrap(),
                builder.get_object("2").unwrap(),
                builder.get_object("3").unwrap(),
                builder.get_object("4").unwrap(),
                builder.get_object("5").unwrap(),
                builder.get_object("6").unwrap(),
                builder.get_object("7").unwrap(),
                builder.get_object("8").unwrap(),
                builder.get_object("9").unwrap(),
            ],
        };
    }
}
