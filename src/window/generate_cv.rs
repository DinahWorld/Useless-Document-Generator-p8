use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn cv(localization: String, compl_adress: String, zipcode: String, city: String, tel: String) {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let name = "Din'sko";
    let adress = "Adresse                    ";
    let tab = "                                 ";
    let xp = format!("{}                 Expérience de travail",tab);


    let font = doc
        .add_external_font(File::open("assets/fonts/Helvetica-Bold.ttf").unwrap())
        .unwrap();
    let font2 = doc
        .add_external_font(File::open("assets/fonts/Helvetica.ttf").unwrap())
        .unwrap();

    // text, font size, x from left edge, y from bottom edge, font
    current_layer.use_text(name, 16, Mm(10.0), Mm(280.0), &font);

    // For more complex layout of text, you can use functions
    // defined on the PdfLayerReference
    // Make sure to wrap your commands
    // in a `begin_text_section()` and `end_text_section()` wrapper
    current_layer.begin_text_section();

    // setup the general fonts.
    // see the docs for these functions for details
    current_layer.set_font(&font2, 14);
    current_layer.set_text_cursor(Mm(10.0), Mm(260.0));
    current_layer.set_line_height(14);
    current_layer.set_word_spacing(3000);
    current_layer.write_text(adress.clone(), &font2);
    current_layer.write_text(localization.clone(), &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab.clone(), &font2);
    current_layer.write_text(compl_adress.clone(), &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab.clone(), &font2);
    current_layer.write_text(zipcode.clone(), &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab.clone(), &font2);
    current_layer.write_text(city.clone(), &font2);
    current_layer.add_line_break();
    current_layer.write_text(tab.clone(), &font2);
    current_layer.write_text(tel.clone(), &font2);
    current_layer.add_line_break();
    current_layer.add_line_break();
    current_layer.write_text(xp.clone(), &font);
    

    current_layer.end_text_section();

    doc.save(&mut BufWriter::new(File::create("../../CV.pdf").unwrap()))
        .unwrap();
}
