extern crate gtk;
pub mod attestation;
pub mod cv;
pub mod resiliation;
use {
    attestation::deplacement as Attestation, cv::cv as Cv, cv::Gender::Femme as F,
    cv::Gender::Homme as H, gtk::prelude::*, resiliation::letter as Letter, std::rc::Rc,
};

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

///Affiche la fenetre principal
fn launch() {
    //On récupere notre fichier glade dans un premier temps
    let glade_src = include_str!("glade/user.glade");
    //On appelle le builder
    let builder = gtk::Builder::from_string(glade_src);

    //On recupere les élements de notre fichier glade
    let window: gtk::Window = builder.get_object("applicationWindow").unwrap();
    let validate_button: gtk::Button = builder.get_object("validateButton").unwrap();

    let homme: gtk::RadioButton = builder.get_object("homme").unwrap();
    let lastname: gtk::Entry = builder.get_object("nom_de_famille").unwrap();
    let firstname: gtk::Entry = builder.get_object("prenom").unwrap();
    let birthday: gtk::Entry = builder.get_object("age").unwrap();
    let born_city: gtk::Entry = builder.get_object("bornCity").unwrap();

    //lorsque l'utilisateur appuiera sur le bouton fermer
    //la fenetre sera détruite
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    //Lorsque le bouton valider sera actionner
    //Le texte entré dans les entry "lastname" "firstname" etc..
    //Sera mis dans la structure User qui va pouvoir acceuillir les différents
    //informations de l'utilisateurs
    validate_button.connect_clicked(clone!(window => move |_| {
        let gender = if homme.get_active() { H } else { F };

        let new_user: Rc<cv::User> = Rc::new(cv::User::new_user(gender,
            lastname.get_text().to_string(),
            firstname.get_text().to_string(),
            birthday.get_text().to_string(),
            born_city.get_text().to_string(),
        ));
        //On cache la fenetre pour pouvoir passer à la fenetre "menu"
        window.hide();
        menu(new_user);
        //Lorsque la fenetre menu sera détruite, on pourra remontrer notre fenetre
        window.show();
    }));

    //On affiche tout les éléments de la fênetre
    window.show_all();

    gtk::main();
}
///Affiche le menu  
pub fn menu(new_user: Rc<cv::User>) {
    //On récupere notre fichier glade dans un premier temps
    let glade_src = include_str!("glade/menu.glade");
    //On appelle le builder
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Menu").unwrap();

    //On recupere les boutons de notre fenetre
    let cv: gtk::Button = builder.get_object("CV").unwrap();
    let attestation: gtk::Button = builder.get_object("Attestation").unwrap();
    let letter: gtk::Button = builder.get_object("Letter").unwrap();

    //lorsque l'utilisateur appuiera sur le bouton fermer
    //la fenetre sera détruite
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    //Lorsque l'utilisateur clickera sur le bouton pour avoir acces
    //a un générateur de CV, on cachera notre fenetre afin de faire apparaitre la fenetre
    //qui permet à l'utilisateur de générer un cv
    cv.connect_clicked(clone!(new_user,window => move |_| {
        window.hide();
        Cv::create_cv(&new_user);
        window.show();
    }));
    //Lorsque l'utilisateur clickera sur le bouton pour avoir acces
    //a un générateur de CV, on cachera notre fenetre afin de faire apparaitre la fenetre
    //qui permet à l'utilisateur de générer une attestation
    attestation.connect_clicked(clone!(new_user,window => move |_| {
        window.hide();
        Attestation::create_attestation(&new_user);
        window.show();
    }));
    //Lorsque l'utilisateur clickera sur le bouton pour avoir acces
    //a un générateur de lettre de résiliation, on cachera notre fenetre afin de
    //faire apparaitre la fenetre qui permet à l'utilisateur de générer une attestation
    letter.connect_clicked(clone!(new_user,window => move |_| {
        window.hide();
        Letter::create_letter(&new_user);
        window.show();
    }));
    window.show_all();

    gtk::main();
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    launch();
}
