use rusty_panther::prelude::*;
use rusty_panther::widgets::{Label, Window};

#[test]
/// Creates a new [`Window`] that has a [`Label`].
fn user_interaction() {
    
    // Create the window and its label
    let mut window = Window::new();

    let mut label1 = Label::builder()
        .border((false, false))
        .text(String::from("This is text."))
        .build();
    let mut label2 = Label::builder()
        .text(String::from("This is text."))
        .build();
    window.grid(Box::new(&mut label1), 1, 1, 1, 1);
    window.grid(Box::new(&mut label2), 2, 1, 1, 1);

    // Run the app (uncomment if you want to have to hit Ctrl+C to continue testing)
    window.run();

    // This makes sure we don't mess up anything in the terminal while testing
    // window.quit();
}