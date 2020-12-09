extern crate gtk;
pub mod window;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    window::user::user();
}
