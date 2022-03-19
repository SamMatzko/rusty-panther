//! Rusty Panther is an easy-to-use terminal GUI crate, using the Termion low-level
//! terminal manipulation library. Once more code gets written, documentation and
//! examples will appear here.

pub mod constants;
pub mod themes;
pub mod traits;
pub mod widgets;

use std::io::{stdin, stdout};

use termion::input::TermRead;
use termion::screen::*;

/// The struct that runs and handles everything. Will have one main window, and any
/// amount of sub-windows.
pub struct Application {
    pub window: widgets::Window,
}
impl Application {

    /// Run the application; this creates the screen and starts the event listener.
    pub fn run() {

        // Create the screen
        AlternateScreen::from(stdout());

        // Start the event listener
        for c in stdin().keys() {
            match c.unwrap() {
                _ => {}
            }
        }
    }
}