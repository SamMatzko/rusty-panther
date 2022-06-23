use rusty_panther::prelude::*;
use rusty_panther::widgets::{Label, Window};

#[test]
/// Creates a new [`Window`] that has a [`Label`].
fn user_interaction() {
    
    // Create the window and its label
    let mut window = Window::new();

    let mut label1 = Label::builder()
        // .border((false, false))
        .text(String::from("Label 1"))
        .build();
    let mut label2 = Label::builder()
        .text(String::from("Label 2"))
        .build();
    let mut label3 = Label::builder()
        .text(String::from("Label 3"))
        .build();
    let mut label4 = Label::builder()
        .text(String::from("Label 4"))
        .build();
    let mut label5 = Label::builder()
        .text(String::from("Label 5"))
        .build();
    let mut label6 = Label::builder()
        .text(String::from("Label 6"))
        .build();
    window.grid(Box::new(&mut label1), 1, 1, 1, 1);
    window.grid(Box::new(&mut label2), 2, 1, 1, 1);
    window.grid(Box::new(&mut label3), 1, 2, 1, 1);
    window.grid(Box::new(&mut label4), 1, 3, 1, 1);
    window.grid(Box::new(&mut label5), 3, 1, 1, 1);

    // Run the app (uncomment if you want to have to hit Ctrl+C to continue testing)
    // window.run();

    // This makes sure we don't mess up anything in the terminal while testing
    window.quit();
}
