use rusty_panther::prelude::*;
use rusty_panther::widgets::{Label, Window};

#[test]
/// Creates a new [`Window`] that has a [`Label`].
fn user_interaction() {
    
    // Create the window and its label
    let mut window = Window::new();

    let mut label1 = Label::builder()
        .set_border((false, false))
        .set_text(String::from("This is text."))
        .build();
    let mut label2 = Label::builder()
        .set_text(String::from("This is text."))
        .build();
    window.add(Box::new(&mut label1), 1, 1);
    window.add(Box::new(&mut label2), 10, 10);

    // Run the app
    window.run();
}