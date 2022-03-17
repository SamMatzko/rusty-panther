//! The module that contains all the widgets used in creating GUIs.

use crate::themes;

use std::io::stdout;
use termion;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::*;

pub struct Window {
    stdout: RawTerminal<std::io::Stdout>,
    theme: themes::Theme,
}
impl Window {
    
    /// Returns the completed [`Window`]; to be called after all the builder functions.
    /// This function also creates the [`AlternateScreen`].
    pub fn build(self) -> Window {
        AlternateScreen::from(stdout());
        Window { stdout: self.stdout, theme: self.theme }
    }

    /// Returns a new [`Window`], which can then be built using the builder functions
    pub fn builder() -> Window {
        let out = stdout().into_raw_mode().unwrap();
        Window { stdout: out, theme: themes::default() }
    }

    // The builder functions. These can be used to optionally customize options.
    // Be sure to call [`build()`] to finalize the creation.

    /// Set the theme for the window.
    pub fn set_theme(&mut self, theme: themes::Theme) -> &mut Window {
        self.theme = theme;
        self
    }
}