use crate::window;
extern crate gtk;
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

pub fn create_cv(user: &Rc<RefCell<window::User>>) {
    let glade_src = include_str!("../glade/cv.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("CV").unwrap();

    let stack_work: Rc<RefCell<Vec<window::Work>>> = Rc::new(RefCell::new(Vec::new()));
    let stack_school: Rc<RefCell<Vec<window::School>>> = Rc::new(RefCell::new(Vec::new()));
    let stack_skill: Rc<RefCell<Vec<window::Skill>>> = Rc::new(RefCell::new(Vec::new()));
    let stack_hobbie: Rc<RefCell<Vec<window::Hobbie>>> = Rc::new(RefCell::new(Vec::new()));

    let localization: gtk::Entry = builder.get_object("adress").unwrap();
    let compl_adress: gtk::Entry = builder.get_object("compl_adress").unwrap();
    let zipcode: gtk::Entry = builder.get_object("zipCode").unwrap();
    let city: gtk::Entry = builder.get_object("city").unwrap();
    let tel: gtk::Entry = builder.get_object("tel").unwrap();

    let date_work: gtk::Entry = builder.get_object("dateWork").unwrap();
    let company: gtk::Entry = builder.get_object("company").unwrap();
    let job: gtk::Entry = builder.get_object("job").unwrap();
    let description_work: gtk::Entry = builder.get_object("descriptionWork").unwrap();
    let add_work: gtk::Button = builder.get_object("addAdress").unwrap();

    let date_school: gtk::Entry = builder.get_object("dateSchool").unwrap();
    let university: gtk::Entry = builder.get_object("university").unwrap();
    let field: gtk::Entry = builder.get_object("field").unwrap();
    let description_school: gtk::Entry = builder.get_object("descriptionSchool").unwrap();
    let add_school: gtk::Button = builder.get_object("addSchool").unwrap();

    let skill: gtk::Entry = builder.get_object("skill").unwrap();
    let level: gtk::Entry = builder.get_object("level").unwrap();
    let add_skill: gtk::Button = builder.get_object("addSkill").unwrap();

    let like: gtk::Entry = builder.get_object("hobbie").unwrap();
    let add_hobbie: gtk::Button = builder.get_object("addHobbie").unwrap();

    let validate: gtk::Button = builder.get_object("validate").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    add_work.connect_clicked(clone!(stack_work => move |_| {
        let wrk = window::Work {
            date_work : date_work.get_text().to_string(),
            company : company.get_text().to_string(),
            job : job.get_text().to_string(),
            description_work : description_work.get_text().to_string(),
        };
        stack_work.borrow_mut().push(wrk);
    }));

    add_school.connect_clicked(clone!(stack_school => move |_| {
        let sch = window::School {
            date_school : date_school.get_text().to_string(),
            university : university.get_text().to_string(),
            field : field.get_text().to_string(),
            description_school : description_school.get_text().to_string(),
        };
        stack_school.borrow_mut().push(sch);
    }));

    add_skill.connect_clicked(clone!(stack_skill => move |_| {
        let skl = window::Skill {
            skill: skill.get_text().to_string(),
            level: level.get_text().to_string(),
        };
        stack_skill.borrow_mut().push(skl);
    }));

    add_hobbie.connect_clicked(clone!(stack_hobbie => move |_| {
        let hb = window::Hobbie {
            like: like.get_text().to_string(),
        };
        stack_hobbie.borrow_mut().push(hb);
    }));

    validate.connect_clicked(
        clone!(user,stack_work,stack_school,stack_skill,stack_hobbie => move |_| {

            let info = window::Adress {
                localization : localization.get_text().to_string(),
                compl_adress : compl_adress.get_text().to_string(),
                zipcode : zipcode.get_text().to_string(),
                city : city.get_text().to_string(),
                tel : tel.get_text().to_string(),

            };
            window::generate_cv::cv(
                &user,
                info,
                &stack_work,
                &stack_school,
                &stack_skill,
                &stack_hobbie);
        }),
    );

    window.show_all();

    gtk::main();
}
