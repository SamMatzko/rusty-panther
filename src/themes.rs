//! The module that contains all the theme-related structs and functions.

use crate::traits::Buildable;

use termion::color::Rgb;

pub struct Theme {
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
}
impl Theme {

    // Builder functions; these can be used to customize theme options, without
    // being required. `build()` must be called at the end to finalize the construction.

    /// Set the background color to the RGB value `bg` of type [`(u8, u8, u8)`].
    pub fn bg_rgb(&mut self, bg: (u8, u8, u8)) -> &mut Theme {
        self.bg = bg;
        self
    }

    /// Set the foreground color to the RGB value `fg` of type [`(u8, u8, u8)`].
    pub fn fg_rgb(&mut self, fg: (u8, u8, u8)) -> &mut Theme {
        self.fg = fg;
        self
    }

    /// Get the background color of this theme as an [`Rgb`]
    pub fn get_bg_rgb(&self) -> Rgb {
        Rgb(self.bg.0, self.bg.1, self.bg.2)
    }

    /// Get the foreground color of this theme as an [`Rgb`].
    pub fn get_fg_rgb(&self) -> Rgb {
        Rgb(self.fg.0, self.fg.1, self.fg.2)
    }
}
impl Buildable for Theme {

    /// Returns the completed [`Theme`] after all builder functions are called
    fn build(self) -> Theme {
        Theme { fg: self.fg, bg: self.bg }
    }

    /// Returns a new [`Theme`], which can then be built using the builder functions
    fn builder() -> Theme {
        Theme { fg: (0, 0, 0), bg: (255, 255, 255) }
    }

    /// Returns a new [`Theme`] with the default configuration. Shorthand for
    /// `Theme::builder().build()`.
    fn new() -> Theme {
        Theme::builder().build()
    }
}

/// Returns the default theme
pub fn default() -> Theme {
    Theme::new()
}