use crate::cv;
use anyhow::Result;
use {
    cv::Adress, cv::User, printpdf::*, std::cell::RefCell, std::fs::File, std::io::BufWriter,
    std::rc::Rc,
};

///Genere un cv
pub fn cv(
    photo: Option<std::path::PathBuf>,
    user: &Rc<User>,
    adress: &Adress,
    stack_work: &Rc<RefCell<Vec<(String, String, String, String)>>>,
    stack_school: &Rc<RefCell<Vec<(String, String, String, String)>>>,
    stack_skill: &Rc<RefCell<Vec<(String, String)>>>,
    stack_hobbie: &Rc<RefCell<Vec<String>>>,
) -> Result<()> {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    //On recupere les informations sous forme de string
    let adress = adress.to_string();

    let name = format!("{} {}", &user.firstname, &user.lastname);

    let tab = "                                   ";
    let inf = format!("{}                          Informations", tab);
    let xp = format!("{}                     Expériences de travail", tab);
    let ed = format!("{}                       Etudes et Diplômes", tab);
    let sk = format!("{}                           Compétences", tab);
    let hb = format!("{}                                 Loisirs", tab);

    //On définit nos polices d'écritures
    let font = doc.add_external_font(File::open("assets/fonts/Helvetica-Bold.ttf")?)?;
    let font2 = doc.add_external_font(File::open("assets/fonts/Helvetica.ttf")?)?;

    //Notre Titre
    current_layer.use_text(name, 16.0, Mm(10.0), Mm(280.0), &font);
    current_layer.add_line_break();
    current_layer.use_text(&user.birthday, 12.0, Mm(10.0), Mm(275.0), &font);

    //Si il y a une photo, on le situera aux coordonnée indiqué
    match photo {
        Some(path) => {
            let photo = path.display().to_string();
            let mut image_file = File::open(photo)?;
            let image = Image::try_from(image::jpeg::JpegDecoder::new(&mut image_file)?)?;
            image.add_to_layer(
                current_layer.clone(),
                Some(Mm(170.0)),
                Some(Mm(250.0)),
                None,
                Some(0.5),
                Some(0.5),
                None,
            );
        }
        None => (),
    }
    //On initialise la zone de texte
    current_layer.begin_text_section();
    //On définit nos parametres par défaut pour ce block de texte
    current_layer.set_font(&font2, 14.0);
    current_layer.set_text_cursor(Mm(10.0), Mm(255.0));
    current_layer.set_line_height(14.0);
    current_layer.set_word_spacing(3000.0);

    current_layer.write_text(inf, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text("Adresse                      ", &font2);
    current_layer.write_text(adress.0, &font2);
    current_layer.add_line_break();
    //Le complément d'adresse est souvent optionel, on l'ignore si l'utilisateur n'en indique pas
    if adress.1 != "" {
        current_layer.write_text(tab, &font2);
        current_layer.write_text(adress.1, &font2);
        current_layer.add_line_break();
    }
    current_layer.write_text(tab, &font2);
    current_layer.write_text(adress.2, &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab, &font2);
    current_layer.write_text(adress.3, &font2);
    current_layer.add_line_break();
    current_layer.write_text("Téléphone                  ", &font2);
    current_layer.write_text(adress.4, &font2);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text(xp, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    //On dépile la pile qui contient les expériences de travail de l'utilisateur
    for work in stack_work.borrow_mut().iter() {
        current_layer.write_text("Entreprise                   ", &font2);
        current_layer.write_text(work.1.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Date                            ", &font2);
        current_layer.write_text(work.0.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Métier                          ", &font2);
        current_layer.write_text(work.2.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Descripion                   ", &font2);
        current_layer.write_text(work.3.clone(), &font2);
        current_layer.add_line_break();
        current_layer.add_line_break();
    }
    current_layer.write_text(ed, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    //On dépile la pile qui contient les études de l'utilisateur
    for school in stack_school.borrow_mut().iter() {
        current_layer.write_text("Etablissement              ", &font2);
        current_layer.write_text(school.1.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Spécialité                     ", &font2);
        current_layer.write_text(school.2.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Durée                           ", &font2);
        current_layer.write_text(school.0.clone(), &font2);
        //Si l'utilisateur n'a pas mis de description, on evitera d'ecrire "description"
        if school.3 != "" {
            current_layer.add_line_break();
            current_layer.write_text("Descripion                    ", &font2);
            current_layer.write_text(school.3.clone(), &font2);
        }
        current_layer.add_line_break();
        current_layer.add_line_break();
    }
    current_layer.write_text(sk, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    //On dépile la pile qui contient les compétences de l'utilisateur
    for skill in stack_skill.borrow_mut().iter() {
        current_layer.write_text("Compétences              ", &font2);
        current_layer.write_text(skill.0.clone(), &font2);
        current_layer.add_line_break();
        current_layer.write_text("Niveau                         ", &font2);
        current_layer.write_text(skill.1.clone(), &font2);
        current_layer.add_line_break();
        current_layer.add_line_break();
    }
    current_layer.write_text(hb, &font);
    current_layer.add_line_break();
    current_layer.add_line_break();
    for hobbie in stack_hobbie.borrow_mut().iter() {
        current_layer.write_text(hobbie.clone(), &font2);
        current_layer.add_line_break();
    }

    current_layer.end_text_section();
    //Si le fichier a bien été généré, grâce à anyhow on recevra "Ok" qui indiquera à l'utilisateur
    //si le fichier a bien été généré
    doc.save(&mut BufWriter::new(File::create("CV.pdf")?))?;
    return Ok(());
}
