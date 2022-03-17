//! The module that contains all the theme-related structs and functions.

pub struct Theme {
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
}
impl Theme {

    /// Returns the completed [`Theme`] after all builder functions are called
    pub fn build(self) -> Theme {
        Theme { fg: self.fg, bg: self.bg }
    }

    /// Returns a new [`Theme`], which can then be built using the builder functions
    pub fn builder() -> Theme {
        Theme { fg: (0, 0, 0), bg: (255, 255, 255) }
    }

    /// Returns a new [`Theme`] with the default configuration
    pub fn default() -> Theme {
        Theme::builder().build()
    }

    // Builder functions; these can be used to customize theme options, without
    // being required. `build()` must be called at the end to finalize the construction.

    /// Set the background hex value to the RGB value `bg` of type [`(u8, u8, u8)`].
    pub fn bg_rgb(&mut self, bg: (u8, u8, u8)) -> &mut Theme {
        self.bg = bg;
        self
    }

    /// Set the foreground hex value to the RGB value `fg` of type [`(u8, u8, u8)`].
    pub fn fg_rgb(&mut self, fg: (u8, u8, u8)) -> &mut Theme {
        self.fg = fg;
        self
    }
}

/// Returns the default theme
pub fn default() -> Theme {
    Theme::default()
}