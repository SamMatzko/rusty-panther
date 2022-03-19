//! Rusty Panther is an easy-to-use terminal GUI crate, using the Termion low-level
//! terminal manipulation library. Once more code gets written, documentation and
//! examples will appear here.

pub mod constants;
pub mod themes;
pub mod traits;
pub mod widgets;

use std::io::{stdin, stdout, Write};

use termion::event::Key;
use termion::input::TermRead;
use termion::screen::*;

use traits::Buildable;

pub fn blah() {
    // Create the screen
    let mut screen = AlternateScreen::from(stdout());
    screen.flush().unwrap();
}

/// The `prelude` module for `rusty_panther`; contains all the necessary traits.
pub mod prelude {
    pub use crate::traits::*;
}

/// The struct that runs and handles everything. Will have one main window, and any
/// amount of sub-windows.
pub struct Application {
    pub window: widgets::Window,
}
impl Application {

    /// Run the application; this creates the screen and starts the event listener.
    pub fn run(&self) {       

        // Start the event listener
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Ctrl('c') => return,
                _ => {}
            }
        }
    }

    // The builder functions. These can be used to optionally customize options.
    // Be sure to call [`build()`] to finalize the creation.

    /// Set the application's window to `window`.
    pub fn set_window(mut self, window: widgets::Window) -> Application {
        self.window = window;
        self
    }
}
impl Buildable for Application {

    fn build(self) -> Application {
        Application { window: self.window }
    }

    fn builder() -> Application {

        Application { window: widgets::Window::new() }
    }

    fn new() -> Application {
        Application::builder().build()
    }
}