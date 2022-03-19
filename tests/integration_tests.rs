use rusty_panther::prelude::*;
use rusty_panther::Application;
use rusty_panther::widgets::{Label, Window};

#[test]
/// Creates a new [`Application`], with a [`Window`] that has a [`Label`].
fn user_interaction() {
    
    // Create the window and its label
    let mut window = Window::new();
    let label = Label::builder()
        .set_text(String::from("This is text."))
        .build();
    window.add(Box::new(label), 100, 100);
    
    // Create the application
    let app = Application::builder()
        .set_window(window)
        .build();

    // Run the app
    app.run();
}