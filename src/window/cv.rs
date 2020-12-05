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

pub fn cv(){

    let glade_src = include_str!("../glade/cv.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("CV").unwrap();

    let stack_adress : Rc<RefCell<Vec<window::Adress>>> = Rc::new(RefCell::new(Vec::new()));
    let adress : gtk::Entry = builder.get_object("adress").unwrap();
    let compl_adress : gtk::Entry = builder.get_object("compl_adress").unwrap();
    let zip_code : gtk::Entry = builder.get_object("zipCode").unwrap();
    let city : gtk::Entry = builder.get_object("city").unwrap();

    let date_work : gtk::Entry = builder.get_object("dateWork").unwrap();
    let company : gtk::Entry = builder.get_object("company").unwrap();
    let job : gtk::Entry = builder.get_object("job").unwrap();
    let description_work : gtk::Entry = builder.get_object("descriptionWork").unwrap();
    let add_adress: gtk::Button = builder.get_object("addAdress").unwrap();

    let tel : gtk::Entry = builder.get_object("tel").unwrap();

    let date_school : gtk::Entry = builder.get_object("dateSchool").unwrap();
    let university : gtk::Entry = builder.get_object("university").unwrap();
    let field : gtk::Entry = builder.get_object("field").unwrap();
    let description_school : gtk::Entry = builder.get_object("descriptionSchool").unwrap();
    let add_school: gtk::Button = builder.get_object("addSchool").unwrap();


    let skill : gtk::Entry = builder.get_object("skill").unwrap();
    let level : gtk::Entry = builder.get_object("level").unwrap();

    let validate : gtk::Button = builder.get_object("validate").unwrap();
    
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    

    add_adress.connect_clicked(clone!(stack_adress => move |_| {
        let adr = window::Adress::create_adress(date_work.clone(),company.clone(),job.clone(),description_school.clone());
        stack_adress.borrow_mut().push(adr);
    }));
    

    validate.connect_clicked(clone!(stack_adress => move |_| {
        for v in &*stack_adress.borrow_mut(){
            println!("{}",v);
        }
    }));

    window.show_all();

    gtk::main();
}