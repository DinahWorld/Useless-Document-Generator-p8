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

pub fn cv() {
    let glade_src = include_str!("../glade/cv.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("CV").unwrap();

    let stack_work: Rc<RefCell<Vec<window::Work>>> = Rc::new(RefCell::new(Vec::new()));
    let stack_school: Rc<RefCell<Vec<window::School>>> = Rc::new(RefCell::new(Vec::new()));
    let stack_skill: Rc<RefCell<Vec<window::Skill>>> = Rc::new(RefCell::new(Vec::new()));

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

    let validate: gtk::Button = builder.get_object("validate").unwrap();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    add_work.connect_clicked(clone!(stack_work => move |_| {
        /*let wrk = window::Work::create_work(
            date_work.clone(),
            company.clone(),
            job.clone(),
            description_work.clone());
        stack_work.borrow_mut().push(wrk);*/
    }));
    
    add_school.connect_clicked(clone!(stack_school => move |_| {
        /*let sch = window::School::create_school(date_school.clone(),
        university.clone(),
        field.clone(),
        description_school.clone());
        stack_school.borrow_mut().push(sch);*/
    }));

    add_skill.connect_clicked(clone!(stack_skill => move |_| {
        /*let skl = window::Skill::create_skill(skill.clone(),level.clone());
        stack_skill.borrow_mut().push(skl);*/
    }));

    validate.connect_clicked(clone!(stack_work,stack_school,stack_skill => move |_| {


        window::generate_cv::cv(
            localization.get_text().to_string(),
            compl_adress.get_text().to_string(),
            zipcode.get_text().to_string(),
            city.get_text().to_string(),
            tel.get_text().to_string());

       /* println!("===== CV ===== ");
        println!("Adresse :");
        let info = window::Adress::create_adress(adress.clone(),compl_adress.clone(),zipcode.clone(),city.clone(),tel.clone());
        println!("{}",info);
        println!("Expérience de Travail :");
        for v in stack_work.borrow_mut().iter(){
            println!("{}",v);
        }
        println!("Etudes :");
        for v in stack_school.borrow_mut().iter(){
            println!("{}",v);
        }
        println!("Compétences :");
        for v in stack_skill.borrow_mut().iter(){
            println!("{}",v);
        }*/
    }));

    window.show_all();

    gtk::main();
}
