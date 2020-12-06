extern crate gtk;
use gtk::prelude::*;
use std::fmt;
pub mod cv;
pub mod generate_cv;
pub mod menu;
pub mod user;

pub enum Gender {
    Homme,
    Femme,
}

pub struct User {
    gender: Gender,
    lastname: String,
    firstname: String,
    age: String,
}
impl User {
    pub fn create_user(
        gender: Gender,
        lastname: gtk::Entry,
        firstname: gtk::Entry,
        age: gtk::Entry,
    ) -> User {
        let lastname = lastname.get_text().to_string();
        let firstname = firstname.get_text().to_string();
        let age = age.get_text().to_string();

        User {
            gender: gender,
            lastname: lastname,
            firstname: firstname,
            age: age,
        }
    }
    pub fn show_id(user: User) {
        println!("{}", user);
    }
}


#[derive(Clone)]
pub struct Adress {
    adress: String,
    compl_adress: String,
    zipcode: String,
    city: String,
    tel: String,
}
/*impl Adress {
    pub fn create_adress(
        adress: gtk::Entry,
        compl_adress: gtk::Entry,
        zipcode: gtk::Entry,
        city: gtk::Entry,
        tel: gtk::Entry,
    ) -> Adress {
        let adress = adress.get_text().to_string();
        let compl_adress = compl_adress.get_text().to_string();
        let zipcode = zipcode.get_text().to_string();
        let city = city.get_text().to_string();
        let tel = tel.get_text().to_string();

        Adress {
            adress: adress,
            compl_adress: compl_adress,
            zipcode: zipcode,
            city: city,
            tel: tel,
        }
    }
}*/

#[derive(Clone)]
pub struct Work {
    date_work: String,
    company: String,
    job: String,
    description_work: String,
}
/*impl Work {
    pub fn create_work(
        date_work: gtk::Entry,
        company: gtk::Entry,
        job: gtk::Entry,
        description_work: gtk::Entry,
    ) -> Work {
        let date_work = date_work.get_text().to_string();
        let company = company.get_text().to_string();
        let job = job.get_text().to_string();
        let description_work = description_work.get_text().to_string();
        Work {
            date_work: date_work,
            company: company,
            job: job,
            description_work: description_work,
        }
    }
}
*/
#[derive(Clone)]
pub struct School {
    date_school: String,
    university: String,
    field: String,
    description_school: String,
}
/*impl School {
    pub fn create_school(
        date_school: gtk::Entry,
        university: gtk::Entry,
        field: gtk::Entry,
        description_school: gtk::Entry,
    ) -> School {
        let date_school = date_school.get_text().to_string();
        let university = university.get_text().to_string();
        let field = field.get_text().to_string();
        let description_school = description_school.get_text().to_string();
        School {
            date_school: date_school,
            university: university,
            field: field,
            description_school: description_school,
        }
    }
}
*/
#[derive(Clone)]
pub struct Skill {
    skill: String,
    level: String,
}
/*impl Skill {
    pub fn create_skill(skill: gtk::Entry, level: gtk::Entry) -> Skill {
        let skill = skill.get_text().to_string();
        let level = level.get_text().to_string();

        Skill {
            skill: skill,
            level: level,
        }
    }
}
*/
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Gender::Homme => write!(f, "Homme"),
            Gender::Femme => write!(f, "Femme"),
        }
    }
}
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "===== Informations =====\nSexe : {}\nNom de Famille : {} \nPrénom : {}\nAge : {}",
            self.gender, self.lastname, self.firstname, self.age
        )
    }
}
/*
impl fmt::Display for Adress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Adresse : {}\nComplément adresse : {}\nCode Postale : {}\nVille : {}\nNuméro de téléphone : {}",
            self.adress, self.compl_adress, self.zipcode, self.city, self.tel
        )
    }
}

impl fmt::Display for Work {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            Date : {}\n
            Entreprise : {}\n
            Métier : {}\n
            Description : {}\n",
            self.date_work, self.company, self.job, self.description_work
        )
    }
}
impl fmt::Display for School {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            Date : {}\n
            Etablissement : {}\n
            Spécialité : {}\n
            Description : {}\n",
            self.date_school, self.university, self.field, self.description_school
        )
    }
}
impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            Compétence : {}\n
            Niveaux : {}\n
            ",
            self.skill, self.level,
        )
    }
}
*/