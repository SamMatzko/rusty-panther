//! The module that contains all the widget structure-related structs. This includes
//! row/column/grid configuration structs and theme structs.

use crate::traits::Buildable;
use crossterm::style::Color;

pub struct Theme {
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
}
impl Theme {

    // Builder functions; these can be used to customize theme options, without
    // being required. `build()` must be called at the end to finalize the construction.

    /// Set the background color to the RGB value `bg` of type [`(u8, u8, u8)`].
    pub fn bg_rgb(mut self, bg: (u8, u8, u8)) -> Theme {
        self.bg = bg;
        self
    }

    /// Set the foreground color to the RGB value `fg` of type [`(u8, u8, u8)`].
    pub fn fg_rgb(mut self, fg: (u8, u8, u8)) -> Theme {
        self.fg = fg;
        self
    }

    /// Get the background color of this theme as an [`Rgb`]
    pub fn get_bg_rgb(&self) -> Color {
        Color::Rgb { r: self.bg.0, g: self.bg.1, b: self.bg.2 }
    }

    /// Get the foreground color of this theme as an [`Rgb`].
    pub fn get_fg_rgb(&self) -> Color {
        Color::Rgb { r: self.fg.0, g: self.fg.1, b: self.fg.2 }
    }
}
impl Buildable for Theme {

    fn build(self) -> Theme {
        Theme { fg: self.fg, bg: self.bg }
    }

    fn builder() -> Theme {
        Theme { fg: (255, 255, 255), bg: (0, 0, 0) }
    }

    fn new() -> Theme {
        Theme::builder().build()
    }
}

/// Returns the default theme
pub fn default() -> Theme {
    Theme::new()
}