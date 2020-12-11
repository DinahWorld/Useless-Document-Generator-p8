use crate::attestation;
use crate::cv;

extern crate gtk;
use attestation::generate_attestation as Generate;
use attestation::Choice;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

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
pub fn true_or_false(vec: &Vec<gtk::CheckButton>) -> usize {
    for i in 0..vec.len() {
        if vec[i].get_active() {
            return i + 1;
        }
    }
    return 0;
}

pub fn create_attestation(user: &Rc<RefCell<cv::User>>) {
    let glade_src = include_str!("../glade/attestation_deplacement.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("Attestation").unwrap();

    let choice = Choice::build(builder.clone());
    let validate: gtk::Button = builder.get_object("validate").unwrap();

    validate.connect_clicked(clone!(user,choice => move |_| {
        Generate::generate_attestation(
            &user,
            &choice,
            true_or_false(&choice.choice));
    }));

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}

/*
#[cfg(test)]
mod tests {

    use super::*;
    use crate::party;
    use party::Instruction as I;
    use party::Orientation as O;
    use party::Robot as R;
    use party::Terrain as T;
    #[test]
    fn test_file() {
        let mut rb = vec![R {
            id: 1,
            x: 1,
            y: 2,
            orientation: O::North,
            instruction: vec![&I::F, &I::L],
        }];

        assert_eq!(file(&mut rb), T { x: 5, y: 5 });
    }
}
*/
