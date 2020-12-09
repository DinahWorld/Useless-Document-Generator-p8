extern crate gtk;
pub mod cv;
pub mod generate_cv;
pub mod menu;
pub mod user;

#[derive(Copy, Clone)]
pub enum Gender {
    Homme,
    Femme,
}
#[derive(Clone)]
pub struct User {
    gender: Gender,
    lastname: String,
    firstname: String,
    birthday: String,
}
#[derive(Clone)]
pub struct Adress {
    localization: String,
    compl_adress: String,
    zipcode: String,
    city: String,
    tel: String,
}

#[derive(Clone)]
pub struct Work {
    date_work: String,
    company: String,
    job: String,
    description_work: String,
}
#[derive(Clone)]
pub struct School {
    date_school: String,
    university: String,
    field: String,
    description_school: String,
}
#[derive(Clone)]
pub struct Skill {
    skill: String,
    level: String,
}
pub struct Hobbie {
    like: String,
}
