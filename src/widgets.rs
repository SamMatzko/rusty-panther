//! The module that contains all the widgets used in creating GUIs.

use crate::themes;
use crate::traits::*;

use std::io::stdout;
use termion;
use termion::raw::{IntoRawMode, RawTerminal};

/// The main window for the terminal application; this contains all the widgets.
/// Usage examples will appear here as soon as a semi-stable release comes out.
pub struct Window {
    stdout: RawTerminal<std::io::Stdout>,
    theme: themes::Theme,
    widgets: Vec<Box<dyn Widget>>,
}
impl Window {

    // The builder functions. These can be used to optionally customize options.
    // Be sure to call [`build()`] to finalize the creation.

    /// Set the theme for the window.
    pub fn set_theme(&mut self, theme: themes::Theme) -> &mut Window {
        self.theme = theme;
        self
    }
}
impl Buildable for Window {

    /// Returns the completed [`Window`]; to be called after all the builder functions.
    /// This function also creates the [`AlternateScreen`].
    fn build(self) -> Window {
        Window { stdout: self.stdout, theme: self.theme, widgets: self.widgets }
    }

    /// Returns a new [`Window`], which can then be built using the builder functions.
    fn builder() -> Window {
        let out = stdout().into_raw_mode().unwrap();
        Window { stdout: out, theme: themes::default(), widgets: Vec::new() }
    }

    /// Returns a new [`Window`] with all the default values. Shorthand for
    /// `Window::builder().build();`.
    fn new() -> Window {
        Window::builder().build()
    }
}