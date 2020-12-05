use crate::window;
extern crate gtk;
use gtk::prelude::*;




pub fn cv(){

    let glade_src = include_str!("../glade/cv.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("CV").unwrap();

    let mut v : Vec<window::Adress> = Vec::new();

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


    let skill : gtk::Entry = builder.get_object("skill").unwrap();
    let level : gtk::Entry = builder.get_object("level").unwrap();


    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    let window_clone = window.clone();
    add_adress.connect_clicked(move |_| {
        let a = window::Adress::create_adress(date_work.clone(),company.clone(),job.clone(),description_school.clone());
        window::Adress::show_adress(a);
    });
    

    window_clone.show_all();

    gtk::main();
}