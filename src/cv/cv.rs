use crate::cv;
extern crate gtk;
use {cv::generate_cv as GenerateCV, cv::User, gtk::prelude::*, std::rc::Rc};

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
///Fonction qui permettra d'afficher la fenetre pour générer un CV
pub fn create_cv(user: &Rc<User>) {
    //On récupere notre fichier glade dans un premier temps
    let glade_src = include_str!("../glade/cv.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("CV").unwrap();

    //On recupere les élements de notre fichier glade
    let stack = cv::Stack::create();
    let add_stack = cv::Addstack::build(builder.clone());
    let cv = cv::Curriculumviter::build(builder.clone());

    let photo: gtk::FileChooserButton = builder.get_object("photo").unwrap();
    let validate: gtk::Button = builder.get_object("validate").unwrap();

    let generated: gtk::Label = builder.get_object("generated").unwrap();

    //Lorsque l'utilisateur clickera sur le bouton pour ajouter une experience de travail
    //On ajoutera dans une pile les informations qu'il a entré
    add_stack.work.connect_clicked(clone!(cv,stack => move |_| {
        let work_string = cv.work.to_string();
        cv.work.clear();
        stack.work.borrow_mut().push(work_string);
    }));
    //Lorsque l'utilisateur clickera sur le bouton pour ajouter des études
    //On ajoutera dans une pile les informations qu'il a entré
    add_stack
        .school
        .connect_clicked(clone!(cv,stack => move |_| {
            let school_string = cv.school.to_string();
            cv.school.clear();
            stack.school.borrow_mut().push(school_string);
        }));
    //Lorsque l'utilisateur clickera sur le bouton pour ajouter des compétences
    //On ajoutera dans une pile les informations qu'il a entré
    add_stack
        .skill
        .connect_clicked(clone!(cv,stack => move |_| {
            let skill_string = cv.skill.to_string();
            cv.skill.clear();
            stack.skill.borrow_mut().push(skill_string);
        }));
    //Lorsque l'utilisateur clickera sur le bouton pour ajouter des hoobbies
    //On ajoutera dans une pile les informations qu'il a entré
    add_stack
        .hobbie
        .connect_clicked(clone!(cv,stack => move |_| {
            let hobbie_string = cv.hobbie.to_string();
            cv.hobbie.clear();
            stack.hobbie.borrow_mut().push(hobbie_string);
        }));
    //Lorsque l'utilisateur aura clické sur le bouton valider, on enverra les informations
    //de l'utilisateur à la fonction "cv" qui va générer le fichier pdf à partir des
    //infos de l'utilisateur
    validate.connect_clicked(clone!(cv,user,stack => move |_| {
        if GenerateCV::cv(
            //Si la fonction retourne ok, on indiquera à l'utilisateur que le fichier a bien été
            //généré
            photo.get_filename(),
            &user,
            &cv.adress,
            &stack.work,
            &stack.school,
            &stack.skill,
            &stack.hobbie).is_ok(){
                generated.set_text("Votre document a été généré 👌");
            }else{
                generated.set_text("Il y a eu un soucis 😱\nAvez-vous bien mis un fichier jpg ?\n(si vous avez mis une photo)");
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
