use crate::attestation;
use crate::cv;

extern crate gtk;
use attestation::generate_attestation as Generate;
use attestation::Choice;
use gtk::prelude::*;
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
///Fonction qui recupere le choix choisi par l'utilisateur entre plusieurs "checkbutton"
pub fn true_or_false(vec: &Vec<gtk::CheckButton>) -> usize {
    for i in 0..vec.len() {
        //Si le bouton est active, la fonction retourne le numéro du bouton
        if vec[i].get_active() {
            return i + 1;
        }
    }
    return 0;
}

///Fonction qui permettra d'afficher la fenetre pour générer une lettre de résiliation
pub fn create_attestation(user: &Rc<cv::User>) {
    //On récupere notre fichier glade dans un premier temps
    let glade_src = include_str!("../glade/attestation_deplacement.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Attestation").unwrap();

    //On recupere les élements de notre fichier glade
    let choice = Choice::build(builder.clone());
    let validate: gtk::Button = builder.get_object("validate").unwrap();
    let generated: gtk::Label = builder.get_object("generated").unwrap();

    //Lorsque l'utilisateur aura clické sur le bouton valider, on enverra les informations
    //de l'utilisateur à la fonction "attestation" qui va générer le fichier pdf à partir des
    //informations de l'utilisateur
    validate.connect_clicked(clone!(user,choice => move |_| {
        //Si la fonction retourne ok, on indiquera à l'utilisateur que le fichier a bien été
        //généré
        if Generate::generate_attestation(
            &user,
            &choice,
            true_or_false(&choice.choice)).is_ok(){
                generated.set_text("Votre document a été généré 👌");
            }else{
                generated.set_text("Il y a eu un soucis 😱");
            }
    }));

    //On affiche tout les éléments de la fênetre
    window.show_all();
    //lorsque l'utilisateur appuiera sur le bouton fermer
    //la fenetre sera détruite
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
