use crate::window;

use {printpdf::*, std::cell::RefCell, std::fs::File, std::io::BufWriter, std::rc::Rc};

pub fn cv(
    user: &Rc<RefCell<window::User>>,
    adress: window::Adress,
    stack_work: &Rc<RefCell<Vec<window::Work>>>,
    stack_school: &Rc<RefCell<Vec<window::School>>>,
    stack_skill: &Rc<RefCell<Vec<window::Skill>>>,
    stack_hobbie: &Rc<RefCell<Vec<window::Hobbie>>>,
) {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let user = user.borrow_mut();
    let name = format!("{} {}", &user.firstname, &user.lastname);

    let tab = "                                   ";
    let inf = format!("{}                          Informations", tab);
    let xp = format!("{}                     Expérience de travail", tab);
    let ed = format!("{}                     Etudes et Diplômes", tab);
    let sk = format!("{}                           Compétences", tab);
    let hb = format!("{}                               Loisirs", tab);

    let font = doc
        .add_external_font(File::open("assets/fonts/Helvetica-Bold.ttf").unwrap())
        .unwrap();
    let font2 = doc
        .add_external_font(File::open("assets/fonts/Helvetica.ttf").unwrap())
        .unwrap();

    current_layer.use_text(name, 16, Mm(10.0), Mm(280.0), &font);
    current_layer.add_line_break();
    current_layer.use_text(&user.birthday, 12, Mm(10.0), Mm(275.0), &font);

    current_layer.begin_text_section();

    current_layer.set_font(&font2, 14);
    current_layer.set_text_cursor(Mm(10.0), Mm(260.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);
    current_layer.write_text(inf, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Adresse                      ", &font2);
    current_layer.write_text(adress.localization, &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab, &font2);
    current_layer.write_text(adress.compl_adress, &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab, &font2);
    current_layer.write_text(adress.zipcode, &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab, &font2);
    current_layer.write_text(adress.city, &font2);
    current_layer.add_line_break();
    current_layer.write_text("Téléphone                  ", &font2);
    current_layer.write_text(adress.tel, &font2);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text(xp, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    for w in stack_work.borrow_mut().iter() {
        current_layer.write_text("Entreprise                   ", &font2);
        current_layer.write_text(w.company.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Date                            ", &font2);
        current_layer.write_text(w.date_work.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Métier                          ", &font2);
        current_layer.write_text(w.job.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Descripion                   ", &font2);
        current_layer.write_text(w.description_work.clone(), &font2);
        current_layer.add_line_break();
        current_layer.add_line_break();
    }
    current_layer.write_text(ed, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    for s in stack_school.borrow_mut().iter() {
        current_layer.write_text("Etablissement              ", &font2);
        current_layer.write_text(s.university.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Spécialité                     ", &font2);
        current_layer.write_text(s.field.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Durée                           ", &font2);
        current_layer.write_text(s.date_school.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Descripion                    ", &font2);
        current_layer.write_text(s.description_school.clone(), &font2);
        current_layer.add_line_break();
        current_layer.add_line_break();
    }
    current_layer.write_text(sk, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    for k in stack_skill.borrow_mut().iter() {
        current_layer.write_text("Compétences              ", &font2);
        current_layer.write_text(k.skill.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Niveau                         ", &font2);
        current_layer.write_text(k.level.clone(), &font2);
        current_layer.add_line_break();
        current_layer.add_line_break();
    }
    current_layer.write_text(hb, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    for h in stack_hobbie.borrow_mut().iter() {
        current_layer.write_text(h.like.clone(), &font2);
        current_layer.add_line_break();
    }
    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create("../../CV.pdf").unwrap()))
        .unwrap();
}
