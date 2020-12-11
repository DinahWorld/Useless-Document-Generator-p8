use crate::cv;
extern crate gtk;
use cv::Adress;
use cv::Hobbie;
use cv::School;
use cv::Skill;
use cv::User;
use cv::Work;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

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

    let stack_work: Rc<RefCell<Vec<(String, String, String, String)>>> =
        Rc::new(RefCell::new(Vec::new()));
    let stack_school: Rc<RefCell<Vec<(String, String, String, String)>>> =
        Rc::new(RefCell::new(Vec::new()));
    let stack_skill: Rc<RefCell<Vec<(String, String)>>> = Rc::new(RefCell::new(Vec::new()));
    let stack_hobbie: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));

    let add_work: gtk::Button = builder.get_object("addAdress").unwrap();
    let add_school: gtk::Button = builder.get_object("addSchool").unwrap();
    let add_skill: gtk::Button = builder.get_object("addSkill").unwrap();
    let add_hobbie: gtk::Button = builder.get_object("addHobbie").unwrap();

    let adress = Adress::build(builder.clone());
    let work = Work::build(builder.clone());
    let school = School::build(builder.clone());
    let skill = Skill::build(builder.clone());
    let hobbie = Hobbie::build(builder.clone());
    let photo : gtk::FileChooserButton = builder.get_object("photo").unwrap();

    let validate: gtk::Button = builder.get_object("validate").unwrap();

    add_work.connect_clicked(clone!(stack_work => move |_| {
        let work_string = work.to_string();
        work.clear();
        stack_work.borrow_mut().push(work_string);
    }));

    add_school.connect_clicked(clone!(stack_school => move |_| {
        let school_string = school.to_string();
        school.clear();
        stack_school.borrow_mut().push(school_string);
    }));

    add_skill.connect_clicked(clone!(stack_skill => move |_| {
        let skill_string = skill.to_string();
        skill.clear();
        stack_skill.borrow_mut().push(skill_string);
    }));

    add_hobbie.connect_clicked(clone!(stack_hobbie => move |_| {
        let hobbie_string = hobbie.to_string();
        hobbie.clear();
        stack_hobbie.borrow_mut().push(hobbie_string);
    }));

    validate.connect_clicked(
        clone!(adress,user,stack_work,stack_school,stack_skill,stack_hobbie => move |_| {
            /*match photo {
                //PathBuf to String
                Some(path) => println!("{}",path.display().to_string()),
                None => (), 
            }*/
            cv::generate_cv::cv(
                photo.get_filename(),
                &user,
                &adress,
                &stack_work,
                &stack_school,
                &stack_skill,
                &stack_hobbie);
        }),
    );

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
