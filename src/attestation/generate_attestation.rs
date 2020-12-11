use crate::attestation;
use crate::cv;

extern crate gtk;
use attestation::Choice;
use chrono::{Datelike, Timelike, Utc};
use cv::User;
use gtk::prelude::*;
use {printpdf::*, std::cell::RefCell, std::fs::File, std::io::BufWriter, std::rc::Rc};

pub fn generate_attestation(user: &Rc<RefCell<User>>, choix: &Choice, choice: usize) {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let now = Utc::now();

    let user = user.borrow_mut();
    let adress = choix.adress.to_string();

    let name = format!("{} {}", &user.firstname, &user.lastname);
    let localization = format!("{} {} {} {}", adress.0, adress.1, adress.2, adress.3);
    let time = format!("à {}:{}", now.hour(), now.minute());
    let day = format!(
        "Le : {}/{}/{}            {}",
        now.day(),
        now.month(),
        now.year(),
        time
    );

    let rules = "          En application du décret no 2020-1310 du 29 octobre 2020 prescrivant les mesures générales nécessaires";
    let rules2 = "                                pour faire face à l’épidémie de COVID-19 dans le cadre de l’état d’urgence sanitaire";
    let font = doc
        .add_external_font(File::open("./assets/fonts/Helvetica-Bold.ttf").unwrap())
        .unwrap();
    let font2 = doc
        .add_external_font(File::open("./assets/fonts/Helvetica-Italic.ttf").unwrap())
        .unwrap();
    let font3 = doc
        .add_external_font(File::open("./assets/fonts/Helvetica.ttf").unwrap())
        .unwrap();
    current_layer.use_text(
        "ATTESTATION DE DÉPLACEMENT DÉROGATOIRE",
        14,
        Mm(45.0),
        Mm(280.0),
        &font,
    );

    current_layer.begin_text_section();
    current_layer.set_font(&font2, 10);
    current_layer.set_text_cursor(Mm(10.0), Mm(270.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);

    current_layer.write_text(rules, &font2);
    current_layer.add_line_break();
    current_layer.write_text(rules2, &font2);
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font3, 11);
    current_layer.set_text_cursor(Mm(10.0), Mm(250.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);

    match &user.gender {
        cv::Gender::Homme => {
            current_layer.write_text("M. : ", &font3);
            current_layer.write_text(name, &font3);
            current_layer.add_line_break();
            current_layer.write_text("Né le : ", &font3);
        }
        cv::Gender::Femme => {
            current_layer.write_text("Mme : ", &font3);
            current_layer.write_text(name, &font3);
            current_layer.add_line_break();
            current_layer.write_text("Née le : ", &font3);
        }
    }
    current_layer.write_text(&user.birthday, &font3);
    current_layer.write_text("   à : ", &font3);
    current_layer.write_text(&user.born_city, &font3);
    current_layer.add_line_break();
    current_layer.write_text("Demeurant : ", &font3);
    current_layer.write_text(localization, &font3);
    current_layer.add_line_break();
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font3, 11);
    current_layer.set_text_cursor(Mm(10.0), Mm(230.0));
    current_layer.set_line_height(11);
    current_layer.set_word_spacing(3000);

    current_layer.write_text("certifie que mon déplacement est lié au motif suivant(cocher la case) autorisé par le décret no 2020-1310 du 29",&font3);
    current_layer.add_line_break();
    current_layer.write_text("octobre 2020 prescrivant les mesures générales nécessaires pour faire face à l’épidémie de COVID-19 dans",&font3);
    current_layer.add_line_break();
    current_layer.write_text("le cadre de l’état d’urgence sanitaire :", &font3);
    current_layer.add_line_break();
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font3, 9);
    current_layer.set_text_cursor(Mm(10.0), Mm(215.0));
    current_layer.set_line_height(10);
    current_layer.set_word_spacing(3000);

    current_layer.write_text("Note : Les personnes souhaitant bénéficier de l’une de ces exceptions doivent se munir s’il y a lieu, lors de leurs déplacements",&font3);
    current_layer.add_line_break();
    current_layer.write_text("hors de leur domicile, d’un document leur permettant de justifier que le déplacement considéré entre dans le champ de l’une de ces",&font3);
    current_layer.add_line_break();
    current_layer.write_text("exception", &font3);
    current_layer.add_line_break();
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font3, 14);
    current_layer.set_text_cursor(Mm(10.0), Mm(180.0));
    current_layer.set_line_height(15);
    current_layer.set_word_spacing(3000);
    match choice {
        1 => {
            current_layer.write_text("Déplacements entre le domicile et le lieu d’exercice de l’activité professionnelle ou un",&font3);
            current_layer.add_line_break();
            current_layer.write_text("ou un établissement d’enseignement ou de formation ; déplacements professionnels ne",&font3);
            current_layer.add_line_break();
            current_layer.write_text(
                "pouvant être différés;déplacements pour un concours ou un examen;",
                &font3,
            );
        }
        2 => {
            current_layer.write_text("Déplacements pour se rendre dans un établissement culturel autorisé ou un lieu de ",&font3);
            current_layer.add_line_break();
            current_layer.write_text("culte ; déplacements pour effectuer des achats de biens, pour des services dont la ",&font3);
            current_layer.add_line_break();
            current_layer.write_text("fourniture est autorisée, pour les retraits de commandes et les livraisons à domicile;",&font3);
        }
        3 => {
            current_layer.write_text(
                "Consultations, examens et soins ne pouvant être assurés à distance et achats",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text("de médicaments;", &font3);
        }
        4 => {
            current_layer.write_text(
                "Déplacements pour motif familial impérieux, pour l’assistance auxpersonnes",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text("vulnérables et précaires ou la garde d’enfants;", &font3);
        }
        5 => {
            current_layer.write_text(
                "Déplacements des personnes en situation de handicap et leur accompagnant;",
                &font3,
            );
        }
        6 => {
            current_layer.write_text(
                "Déplacements en plein air ou vers un lieu de plein air, sans changement ",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "du lieu de résidence, dans la limite de trois heures quotidiennes et dans",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "un rayon maximal de vingt kilomètres autour du domicile, liés soit à ",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "l’activité physique ou aux loisirs individuels, à l’exclusion de toute",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "pratique sportive collective et de toute proximité avec d’autres personnes,",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "soit à la promenade avec les seules personnes regroupées dans un même",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "domicile, soit aux besoins des animaux de compagnie;",
                &font3,
            );
        }
        7 => {
            current_layer.write_text(
                "Convocations judiciaires ou administratives et déplacements pour se rendre",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text("dans un service public;", &font3);
        }
        8 => {
            current_layer.write_text("Participation à des missions d’intérêt général sur demandede l’autorité administrative;",&font3);
            current_layer.add_line_break();
            current_layer.write_text("administrative;", &font3);
        }
        9 => {
            current_layer.write_text(
                "Déplacements pour chercher les enfants à l’école et à l’occasion de leurs ",
                &font3,
            );
            current_layer.add_line_break();
            current_layer.write_text("activités périscolaires;", &font3);
        }
        _ => (),
    }
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font3, 14);
    current_layer.set_text_cursor(Mm(120.0), Mm(60.0));
    current_layer.set_line_height(20);
    current_layer.set_word_spacing(3000);
    current_layer.write_text("Date et heure de sortie : ", &font3);
    current_layer.write_text(choix.hour.get_text().to_string(), &font3);

    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font3, 11);
    current_layer.set_text_cursor(Mm(10.0), Mm(20.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);

    current_layer.write_text("Fait à : ", &font3);
    current_layer.write_text(adress.3, &font3);
    current_layer.add_line_break();
    current_layer.write_text(day, &font3);
    current_layer.add_line_break();
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(
        File::create("attestation.pdf").unwrap(),
    ))
    .unwrap();
}
