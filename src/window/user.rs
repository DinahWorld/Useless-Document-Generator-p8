use crate::window;
extern crate gtk;
use gtk::prelude::*;

pub fn user() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    
    let glade_src = include_str!("../glade/test.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("applicationWindow").unwrap();
    let validate_button: gtk::Button = builder.get_object("validateButton").unwrap();
    let homme: gtk::RadioButton = builder.get_object("homme").unwrap();
    let femme: gtk::RadioButton = builder.get_object("femme").unwrap();

    let lastname: gtk::Entry = builder.get_object("nom_de_famille").unwrap();
    let firstname: gtk::Entry = builder.get_object("prenom").unwrap();
    let age: gtk::Entry = builder.get_object("age").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let window_clone = window.clone();
    validate_button.connect_clicked(move |_| {
        let mut gender = window::Gender::Homme;
        if femme.get_active() {
            gender = window::Gender::Femme;
        }
        if homme.get_active() {
            gender = window::Gender::Homme;
        }

        let new_user =
        window::User::create_user(gender, lastname.clone(), firstname.clone(), age.clone());
        window::User::show_id(new_user);
        window.hide();
        window::menu::menu();
    });

    window_clone.show_all();

    gtk::main();
}
