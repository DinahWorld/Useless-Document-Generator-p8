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

pub fn user() {
    let glade_src = include_str!("../glade/test.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("applicationWindow").unwrap();
    let validate_button: gtk::Button = builder.get_object("validateButton").unwrap();
    let homme: gtk::RadioButton = builder.get_object("homme").unwrap();
    let femme: gtk::RadioButton = builder.get_object("femme").unwrap();

    let lastname: gtk::Entry = builder.get_object("nom_de_famille").unwrap();
    let firstname: gtk::Entry = builder.get_object("prenom").unwrap();
    let birthday: gtk::Entry = builder.get_object("age").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    validate_button.connect_clicked(clone!(window => move |_| {
        let mut gender = window::Gender::Homme;
        if femme.get_active() {
            gender = window::Gender::Femme;
        }
        if homme.get_active() {
            gender = window::Gender::Homme;
        }

        let new_user = window::User{
            gender : gender,
            lastname : lastname.get_text().to_string(),
            firstname : firstname.get_text().to_string(),
            birthday : birthday.get_text().to_string(),
        };
        window.hide();
        window::menu::menu(new_user);
        window.show();
    }));

    window.show_all();

    gtk::main();
}
