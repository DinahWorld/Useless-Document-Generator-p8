use crate::cv;
extern crate gtk;
use {cv::generate_cv as GenerateCV, cv::User, gtk::prelude::*, std::cell::RefCell, std::rc::Rc};

//https://gtk-rs.org/docs-src/tutorial/closures
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
pub fn create_cv(user: &Rc<RefCell<User>>) {
    let glade_src = include_str!("../glade/cv.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("CV").unwrap();

    let stack = cv::Stack::create();
    let add_stack = cv::Addstack::build(builder.clone());
    let cv = cv::Curriculumviter::build(builder.clone());

    let photo: gtk::FileChooserButton = builder.get_object("photo").unwrap();
    let validate: gtk::Button = builder.get_object("validate").unwrap();

    add_stack.work.connect_clicked(clone!(cv,stack => move |_| {
        let work_string = cv.work.to_string();
        cv.work.clear();
        stack.work.borrow_mut().push(work_string);
    }));

    add_stack
        .school
        .connect_clicked(clone!(cv,stack => move |_| {
            let school_string = cv.school.to_string();
            cv.school.clear();
            stack.school.borrow_mut().push(school_string);
        }));

    add_stack
        .skill
        .connect_clicked(clone!(cv,stack => move |_| {
            let skill_string = cv.skill.to_string();
            cv.skill.clear();
            stack.skill.borrow_mut().push(skill_string);
        }));

    add_stack
        .hobbie
        .connect_clicked(clone!(cv,stack => move |_| {
            let hobbie_string = cv.hobbie.to_string();
            cv.hobbie.clear();
            stack.hobbie.borrow_mut().push(hobbie_string);
        }));

    validate.connect_clicked(clone!(cv,user,stack => move |_| {
        GenerateCV::cv(
            photo.get_filename(),
            &user,
            &cv.adress,
            &stack.work,
            &stack.school,
            &stack.skill,
            &stack.hobbie);
    }));

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
