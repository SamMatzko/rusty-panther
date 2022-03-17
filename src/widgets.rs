//! The module that contains all the widgets used in creating GUIs.

use crate::theme;

use std::io::{stdout, Write};
use termion;
use termion::raw::IntoRawMode;
use termion::screen::*;

// termion::screen::AlternateScreen<std::io::stdio::Stdout>

pub struct Window<W: Write> {
    screen: AlternateScreen<W>,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}
impl<W: Write> Window<W> {
    
    /// Returns the completed [`Window`]; to be called after all the builder functions
    pub fn build(self) -> Window<W> {
        Window { screen: self.screen, stdout: self.stdout }
    }

    /// Returns a new [`Window`], which can then be built using the builder functions
    pub fn builder() -> Window<W> {
        let out = stdout().into_raw_mode().unwrap();
        Window { screen: AlternateScreen::from(out), stdout: out, }
    }

    // The builder functions. These 
}