extern crate gtk;
use crate::attestation;
use crate::cv;
use crate::resiliation;
use {
    attestation::deplacement as Attestation, cv::User, gtk::prelude::*,
    resiliation::generate_letter as Generate, resiliation::Letter, std::rc::Rc,
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
///Fonction qui permettra d'afficher la fenetre pour générer une lettre de résiliation
pub fn create_letter(user: &Rc<User>) {
    //On récupere notre fichier glade dans un premier temps
    let glade_src = include_str!("../glade/lettre_resiliation.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Lettre").unwrap();
    //On recupere les élements de notre fichier glade
    let letter = Letter::build(builder.clone());
    let validate: gtk::Button = builder.get_object("validate").unwrap();
    let generated: gtk::Label = builder.get_object("generated").unwrap();

    //Lorsque l'utilisateur aura clické sur le bouton valider, on enverra les informations
    //de l'utilisateur à la fonction "letter" qui va générer le fichier pdf à partir des
    //informations de l'utilisateur
    validate.connect_clicked(clone!(letter,user => move |_| {
        //Si la fonction retourne ok, on indiquera à l'utilisateur que le fichier a bien été
        //généré
        if Generate::letter(
            &user,
            &letter,
            Attestation::true_or_false(&letter.internet_box)
        ).is_ok(){
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
