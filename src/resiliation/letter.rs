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

pub fn create_letter(user: &Rc<User>) {
    let glade_src = include_str!("../glade/lettre_resiliation.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Lettre").unwrap();

    let letter = Letter::build(builder.clone());
    let validate: gtk::Button = builder.get_object("validate").unwrap();
    let generated: gtk::Label = builder.get_object("generated").unwrap();

    validate.connect_clicked(clone!(letter,user => move |_| {
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

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
