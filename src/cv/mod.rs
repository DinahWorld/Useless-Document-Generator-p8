extern crate gtk;
use gtk::prelude::*;
pub mod cv;
pub mod generate_cv;
use std::cell::RefCell;
use std::rc::Rc;
#[macro_export]
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

#[derive(Clone)]
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
    pub born_city: String,
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
#[derive(Clone)]
pub struct Curriculumviter {
    adress: Adress,
    work: Work,
    school: School,
    skill: Skill,
    hobbie: Hobbie,
}
#[derive(Clone)]
pub struct Stack {
    work: Rc<RefCell<Vec<(String, String, String, String)>>>,
    school: Rc<RefCell<Vec<(String, String, String, String)>>>,
    skill: Rc<RefCell<Vec<(String, String)>>>,
    hobbie: Rc<RefCell<Vec<String>>>,
}
pub struct Addstack {
    work: gtk::Button,
    school: gtk::Button,
    skill: gtk::Button,
    hobbie: gtk::Button,
}

impl User {
    ///Va contenir les informations de l'utilisateurs dans la structure User sous forme de string
    pub fn new_user(
        gender: Gender,
        lastname: String,
        firstname: String,
        birthday: String,
        born_city: String,
    ) -> User {
        return User {
            gender: gender,
            lastname: lastname,
            firstname: firstname,
            birthday: birthday,
            born_city: born_city,
        };
    }
}
impl Adress {
    ///Met dans un tupple les informations de l'utilisateur en String
    pub fn to_string(&self) -> (String, String, String, String, String) {
        let localization = self.localization.get_text().to_string();
        let compl_adress = self.compl_adress.get_text().to_string();
        let zipcode = self.zipcode.get_text().to_string();
        let city = self.city.get_text().to_string();
        let tel = self.tel.get_text().to_string();

        return (localization, compl_adress, zipcode, city, tel);
    }
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    pub fn build(builder: gtk::Builder) -> Adress {
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
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    fn build(builder: gtk::Builder) -> Work {
        return Work {
            date_work: builder.get_object("dateWork").unwrap(),
            company: builder.get_object("company").unwrap(),
            job: builder.get_object("job").unwrap(),
            description_work: builder.get_object("descriptionWork").unwrap(),
        };
    }
    ///Efface le texte entré
    fn clear(&self) {
        self.date_work.set_text("");
        self.company.set_text("");
        self.job.set_text("");
        self.description_work.set_text("");
    }
}

impl School {
    ///Met dans un tupple les informations de l'utilisateur en String
    fn to_string(&self) -> (String, String, String, String) {
        let date_school = self.date_school.get_text().to_string();
        let university = self.university.get_text().to_string();
        let field = self.field.get_text().to_string();
        let description_school = self.description_school.get_text().to_string();

        return (date_school, university, field, description_school);
    }
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    fn build(builder: gtk::Builder) -> School {
        return School {
            date_school: builder.get_object("dateSchool").unwrap(),
            university: builder.get_object("university").unwrap(),
            field: builder.get_object("field").unwrap(),
            description_school: builder.get_object("descriptionSchool").unwrap(),
        };
    }
    ///Efface le texte entré
    fn clear(&self) {
        self.date_school.set_text("");
        self.university.set_text("");
        self.field.set_text("");
        self.description_school.set_text("");
    }
}
impl Skill {
    ///Met dans un tupple les informations de l'utilisateur en String
    fn to_string(&self) -> (String, String) {
        let thing = self.thing.get_text().to_string();
        let level = self.level.get_text().to_string();

        return (thing, level);
    }
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    fn build(builder: gtk::Builder) -> Skill {
        return Skill {
            thing: builder.get_object("skill").unwrap(),
            level: builder.get_object("level").unwrap(),
        };
    }
    ///Efface le texte entré
    fn clear(&self) {
        self.thing.set_text("");
        self.level.set_text("");
    }
}
impl Hobbie {
    ///Met dans un tupple les informations de l'utilisateur en String
    fn to_string(&self) -> String {
        let like = self.like.get_text().to_string();

        return like;
    }
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    fn build(builder: gtk::Builder) -> Hobbie {
        return Hobbie {
            like: builder.get_object("hobbie").unwrap(),
        };
    }
    ///Efface le texte entré
    fn clear(&self) {
        self.like.set_text("");
    }
}

impl Curriculumviter {
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    pub fn build(builder: gtk::Builder) -> Curriculumviter {
        return Curriculumviter {
            adress: Adress::build(builder.clone()),
            school: School::build(builder.clone()),
            work: Work::build(builder.clone()),
            skill: Skill::build(builder.clone()),
            hobbie: Hobbie::build(builder.clone()),
        };
    }
}

impl Addstack {
    ///Va "builder.get_object("x").unwrap()," chaque variable de la structure afin de recuperer chaque élément de l'interface  
    pub fn build(builder: gtk::Builder) -> Addstack {
        return Addstack {
            work: builder.get_object("addAdress").unwrap(),
            school: builder.get_object("addSchool").unwrap(),
            skill: builder.get_object("addSkill").unwrap(),
            hobbie: builder.get_object("addHobbie").unwrap(),
        };
    }
}
impl Stack {
    /// Rc::new(RefCell::new(Vec::new()))
    pub fn create() -> Stack {
        return Stack {
            work: Rc::new(RefCell::new(Vec::new())),
            school: Rc::new(RefCell::new(Vec::new())),
            skill: Rc::new(RefCell::new(Vec::new())),
            hobbie: Rc::new(RefCell::new(Vec::new())),
        };
    }
}
