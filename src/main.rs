extern crate gtk;
pub mod attestation;
pub mod cv;
pub mod resiliation;
use {
    attestation::deplacement as Attestation, cv::cv as Cv, cv::Gender::Femme as F,
    cv::Gender::Homme as H, gtk::prelude::*, resiliation::letter as Letter, std::cell::RefCell,
    std::rc::Rc,
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

pub fn menu(new_user: cv::User) {
    let glade_src = include_str!("glade/menu.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Menu").unwrap();

    let cv: gtk::Button = builder.get_object("CV").unwrap();
    let attestation: gtk::Button = builder.get_object("Attestation").unwrap();
    let letter: gtk::Button = builder.get_object("Letter").unwrap();

    let user: Rc<RefCell<cv::User>> = Rc::new(RefCell::new(new_user));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    cv.connect_clicked(clone!(user,window => move |_| {
        window.hide();
        Cv::create_cv(&user);
        window.show();
    }));
    attestation.connect_clicked(clone!(user,window => move |_| {
        window.hide();
        Attestation::create_attestation(&user);
        window.show();
    }));
    letter.connect_clicked(clone!(user,window => move |_| {
        window.hide();
        Letter::create_letter(&user);
        window.show();
    }));
    window.show_all();

    gtk::main();
}

fn user() {
    let glade_src = include_str!("glade/user.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("applicationWindow").unwrap();
    let validate_button: gtk::Button = builder.get_object("validateButton").unwrap();

    let homme: gtk::RadioButton = builder.get_object("homme").unwrap();
    let lastname: gtk::Entry = builder.get_object("nom_de_famille").unwrap();
    let firstname: gtk::Entry = builder.get_object("prenom").unwrap();
    let birthday: gtk::Entry = builder.get_object("age").unwrap();
    let born_city: gtk::Entry = builder.get_object("bornCity").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    validate_button.connect_clicked(clone!(window => move |_| {
        let mut gender = F;
        if homme.get_active() {
            gender = H;
        }

        let new_user = cv::User::new_user(gender,lastname.clone(),firstname.clone(),birthday.clone(),born_city.clone());
        window.hide();
        menu(new_user);
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
    user();
}
