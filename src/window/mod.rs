extern crate gtk;
use gtk::prelude::*;
pub mod cv;
pub mod generate_cv;

#[derive(Copy, Clone)]
pub enum Gender {
    Homme,
    Femme,
}
#[derive(Clone)]
pub struct User {
    pub gender: Gender,
    pub lastname: String,
    pub firstname: String,
    pub birthday: String,
}
#[derive(Clone)]
pub struct Adress {
    localization: gtk::Entry,
    compl_adress: gtk::Entry,
    zipcode: gtk::Entry,
    city: gtk::Entry,
    tel: gtk::Entry,
}
#[derive(Clone)]
pub struct Work {
    date_work: gtk::Entry,
    company: gtk::Entry,
    job: gtk::Entry,
    description_work: gtk::Entry,
}
#[derive(Clone)]
pub struct School {
    date_school: gtk::Entry,
    university: gtk::Entry,
    field: gtk::Entry,
    description_school: gtk::Entry,
}
#[derive(Clone)]
pub struct Skill {
    thing: gtk::Entry,
    level: gtk::Entry,
}
#[derive(Clone)]
pub struct Hobbie {
    like: gtk::Entry,
}



impl Adress {
    ///Met dans un tupple les informations de l'utilisateur en String
    fn to_string(&self) -> (String, String, String, String, String) {
        let localization = self.localization.get_text().to_string();
        let compl_adress = self.compl_adress.get_text().to_string();
        let zipcode = self.zipcode.get_text().to_string();
        let city = self.city.get_text().to_string();
        let tel = self.tel.get_text().to_string();

        return (localization, compl_adress, zipcode, city, tel);
    }
    
    fn build(builder : gtk::Builder) -> Adress {
        return Adress {
            localization: builder.get_object("adress").unwrap(),
            compl_adress: builder.get_object("compl_adress").unwrap(),
            zipcode: builder.get_object("zipCode").unwrap(),
            city: builder.get_object("city").unwrap(),
            tel: builder.get_object("tel").unwrap(),
        };
    }
}
impl Work {
    ///Met dans un tupple les informations de l'utilisateur en String
    fn to_string(&self) -> (String, String, String, String) {
        let date_work = self.date_work.get_text().to_string();
        let company = self.company.get_text().to_string();
        let job = self.job.get_text().to_string();
        let description_work = self.description_work.get_text().to_string();

        return (date_work, company, job, description_work);
    }
    fn build(builder : gtk::Builder) -> Work {
        return Work {
            date_work: builder.get_object("dateWork").unwrap(),
            company: builder.get_object("company").unwrap(),
            job: builder.get_object("job").unwrap(),
            description_work: builder.get_object("descriptionWork").unwrap(),
        };
    }
    ///Efface le texte entré
    fn clear(&self){
        self.date_work.set_text("");
        self.company.set_text("");
        self.job.set_text("");
        self.description_work.set_text("");
    }
}

impl School {
    ///Met dans un tupple les informations de l'utilisateur en String
    fn to_string(&self) -> (String,String,String,String) {
        let date_school = self.date_school.get_text().to_string();
        let university = self.university.get_text().to_string();
        let field = self.field.get_text().to_string();
        let description_school = self.description_school.get_text().to_string();

        return (date_school,university,field,description_school);
    }
    fn build(builder : gtk::Builder) -> School {
        return School {
            date_school : builder.get_object("dateSchool").unwrap(),
            university : builder.get_object("university").unwrap(),
            field : builder.get_object("field").unwrap(),
            description_school : builder.get_object("descriptionSchool").unwrap(),
        };
    }
    ///Efface le texte entré
     fn clear(&self){
        self.date_school.set_text("");
        self.university.set_text("");
        self.field.set_text("");
        self.description_school.set_text("");
    }
}
impl Skill {
    fn to_string(&self) -> (String,String) {
        let thing = self.thing.get_text().to_string();
        let level = self.level.get_text().to_string();

        return (thing,level);
    }
    fn build(builder : gtk::Builder) -> Skill {
        return Skill {
            thing : builder.get_object("skill").unwrap(),
            level : builder.get_object("level").unwrap(),
        };
    }
    ///Efface le texte entré
    fn clear(&self){
        self.thing.set_text("");
        self.level.set_text("");
    }
}
impl Hobbie {
    fn to_string(&self) -> String {
        let like = self.like.get_text().to_string();

        return like;
    }
    fn build(builder : gtk::Builder) -> Hobbie {
        return Hobbie {
            like : builder.get_object("hobbie").unwrap(),
           
        };
    }
    ///Efface le texte entré
    fn clear(&self) {
        self.like.set_text("");
    }
}

