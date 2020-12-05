extern crate gtk;
use gtk::prelude::*;
use std::fmt;
pub mod menu;
pub mod user;
pub mod cv;


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
pub struct Adress {
    date_work: String,
    company: String,
    job: String,
    description_work: String,
}
impl Adress {
    pub fn create_adress(
        date_work: gtk::Entry,
        company: gtk::Entry,
        job: gtk::Entry,
        description_work: gtk::Entry,
    ) -> Adress {

        let date_work = date_work.get_text().to_string();
        let company = company.get_text().to_string();
        let job = job.get_text().to_string();
        let description_work = description_work.get_text().to_string();
        Adress {
            date_work: date_work,
            company: company,
            job: job,
            description_work: description_work,
        }
    }

    pub fn show_adress(adress: Adress){
        println!("{}",adress)
    }
}

pub enum Gender {
    Homme,
    Femme,
}
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
impl fmt::Display for Adress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Adresse {} {} {} {}",
            self.date_work, self.company, self.job, self.description_work
        )
    }
}
