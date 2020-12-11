use {printpdf::*, std::cell::RefCell, std::fs::File, std::io::BufWriter, std::rc::Rc};
extern crate gtk;
use crate::cv;
use crate::resiliation;
use cv::User;
use gtk::prelude::*;
use resiliation::Letter;

pub fn letter(user: &Rc<RefCell<User>>, letter: &Letter, box_internet: usize) {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let user = user.borrow_mut();
    let adress = letter.adress.to_string();
    let email = letter.email.get_text().to_string();
    //let box_internet = letter.internet_box.get_text().to_string();
    let tel = letter.tel.get_text().to_string();
    let suscribe_nb = letter.suscribe_nb.get_text().to_string();

    let name = format!("{} {}", &user.firstname, &user.lastname);
    //let boxx = format!("Lettre de Résiliation {}.pdf",box_internet);
    let boxx = format!("Lettre de Résiliation.pdf");
    let font = doc
        .add_external_font(File::open("assets/fonts/Helvetica.ttf").unwrap())
        .unwrap();

    current_layer.begin_text_section();
    current_layer.set_font(&font, 12);
    current_layer.set_text_cursor(Mm(10.0), Mm(280.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);

    current_layer.write_text(name.clone(), &font);
    current_layer.add_line_break();
    current_layer.write_text(adress.0, &font);
    current_layer.add_line_break();
    current_layer.write_text(adress.1, &font);
    current_layer.add_line_break();
    current_layer.write_text(adress.2, &font);
    current_layer.add_line_break();
    current_layer.write_text(adress.3, &font);
    current_layer.add_line_break();
    current_layer.write_text(email, &font);
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font, 12);
    current_layer.set_text_cursor(Mm(150.0), Mm(250.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);
    match box_internet {
        1 => {
            current_layer.write_text("SFR - Service Résiliation", &font);
            current_layer.add_line_break();
            current_layer.write_text("TSA 30103", &font);
            current_layer.add_line_break();
            current_layer.write_text("69947 Lyon Cedex 20", &font);
        }
        2 => {
            current_layer.write_text("Service client RED by SFR", &font);
            current_layer.add_line_break();
            current_layer.write_text("TSA 30103", &font);
            current_layer.add_line_break();
            current_layer.write_text("69947 Lyon Cedex 20", &font);
        }
        3 => {
            current_layer.write_text("Sosh – Service clients", &font);
            current_layer.add_line_break();
            current_layer.write_text("33734 Bordeaux Cedex 9", &font);
        }
        4 => {
            current_layer.write_text("Orange Service Clients Internet", &font);
            current_layer.add_line_break();
            current_layer.write_text("TSA 10008", &font);
            current_layer.add_line_break();
            current_layer.write_text("59878 Lille Cedex 9", &font);
        }
        5 => {
            current_layer.write_text("FREE résiliation", &font);
            current_layer.add_line_break();
            current_layer.write_text("BP 40090", &font);
            current_layer.add_line_break();
            current_layer.write_text("91003 Evry cedex", &font);
        }
        _ => (),
    }
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font, 14);
    current_layer.set_text_cursor(Mm(10.0), Mm(215.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);

    current_layer.write_text("Objet : Résiliation du contrat d'abonnement", &font);
    current_layer.end_text_section();

    current_layer.begin_text_section();
    current_layer.set_font(&font, 12);
    current_layer.set_text_cursor(Mm(10.0), Mm(205.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);

    current_layer.write_text("Numéro de ligne téléphonique : ", &font);
    current_layer.write_text(tel, &font);
    current_layer.add_line_break();
    current_layer.write_text("Numéro Abonné associé à la ligne téléphonique : ", &font);
    current_layer.write_text(suscribe_nb, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Madame, Monsieur", &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Par la présente, je vous informe de ma décision de résilier mon contrat d’abonnement au service",&font);
    current_layer.add_line_break();
    current_layer.write_text("d'accès internet souscrit auprès de votre société et vous prie de bien vouloir procéder à la résiliation ",&font);
    current_layer.add_line_break();
    current_layer.write_text(
        "de mon contrat d’abonnement au terme du délai légal de 10 jours.",
        &font,
    );
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Je vous remercie de m'adresser une confirmation écrite précisant la date effective de résiliation",&font);
    current_layer.add_line_break();
    current_layer.write_text("ainsi qu'une facture de clôture de compte.", &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Dans cette attente, je vous prie d'agréer, Madame, Monsieur, l'expression de mes salutations",&font);
    current_layer.add_line_break();
    current_layer.write_text("distinguées", &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text(name, &font);
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create(boxx).unwrap()))
        .unwrap();
}
