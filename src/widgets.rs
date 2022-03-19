//! The module that contains all the widgets used in creating GUIs.

use crate::constants::chars;
use crate::themes;
use crate::traits::*;

use std::io::{stdout, Write};

use termion;
use termion::color::{Bg, Fg, Reset, Rgb};
use termion::cursor;
use termion::raw::{IntoRawMode, RawTerminal};

/// A function that creates a border box
fn create_border_box(x: u16, y: u16, width: u16, height: u16, fg: Rgb, bg: Rgb) {
    
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    // Create the top of the box
    write!(
        stdout,
        "{}{}{}{}{}{}{}{}",
        cursor::Goto(x, y),
        Fg(fg),
        Bg(bg),
        chars::TOP_LEFT,
        chars::HORIZONTAL.repeat((width - 2) as usize),
        chars::TOP_RIGHT,
        Fg(Reset),
        Bg(Reset)
    ).unwrap();

    // Create all the sides
    for i in 0..(height - 2) {
        write!(
            stdout,
            "{}{}{}{}{}{}{}{}",
            cursor::Goto(x, y + (i + 1)),
            Fg(fg),
            Bg(bg),
            chars::VERTICAL,
            chars::EMPTY.repeat((width - 2) as usize),
            chars::VERTICAL,
            Fg(Reset),
            Bg(Reset)
        ).unwrap();
    }

    // Create the bottom of the box
    write!(
        stdout,
        "{}{}{}{}{}{}{}{}",
        cursor::Goto(x, y + height - 1),
        Fg(fg),
        Bg(bg),
        chars::BOTTOM_LEFT,
        chars::HORIZONTAL.repeat((width - 2) as usize),
        chars::BOTTOM_RIGHT,
        Fg(Reset),
        Bg(Reset)
    ).unwrap();

    stdout.flush().unwrap();
}

/// A function that creates a filled, borderless box
fn create_fill_box(x: u16, y: u16, width: u16, height: u16, bg: Rgb) {

    // Simply write the color to each row
    for h in 0..height {
        write!(
            stdout,
            "{}{}{}",
            cursor::Goto(x, y + h),
            Bg(bg),
            chars::EMPTY.repeat(x as usize)
        ).unwrap();
    }
}

/// A simple label widget for displaying text.
pub struct Label {
    /// A tuple containg two [`bool`]s; whether there is a border, and whether to
    /// show the border
    border: (bool, bool),
    /// The stdout to which all the widgets are printed.
    stdout: RawTerminal<std::io::Stdout>,
    /// The text that the label contains
    text: String,
    /// The [`themes::Theme`] that this label uses for it's colors
    theme: themes::Theme,
    /// The theme that this label uses. Can be overriden by 
    /// The width of the label, in chars
    width: u16,
}
impl Buildable for Label {

    /// Returns the completed [`Label`]; to be called after all the builder functions.
    fn build(self) -> Label {
        Label {
            border: self.border,
            stdout: self.stdout,
            text: self.text,
            theme: self.theme,
            width: self.width
        }
    }

    /// Returns a new [`Label`], which can then be built using the builder functions.
    fn builder() -> Label {
        Label {
            border: (true, true),
            stdout: stdout().into_raw_mode().unwrap(),
            text: String::from(""),
            theme: themes::default(),
            width: 10
        }
    }

    /// Returns a new [`Label`] with the default values. Shorthand for
    /// `Label::builder().build();`.
    fn new() -> Label {
        Label::builder().build()
    }
}
impl Widget for Label {

    /// Draw the label, at the given coordinates. This function is called by parents.
    /// `width` is not relevant here, for now.
    fn draw(&self, x: u16, y: u16, _width: u16, height: u16) {

        // The positioning of the text
        let mut text_x: u16 = x;
        let mut text_y: u16 = y;

        // Create the background box, and if there needs to be a border, create
        // the border.
        if self.border.0 {

            // Make sure the text doesn't end up on the border
            text_x += 1;
            text_y += 1;

            // Create the bordered box
            create_border_box(
                x,
                y,
                self.width + 1,
                height + 2,
                self.theme.get_fg_rgb(),
                self.theme.get_bg_rgb()
            );
        }
        else {
            
            // Create the unbordered box
            create_fill_box(
                x,
                y,
                self.width,
                height,
                self.theme.get_bg_rgb()
            );
        }

        // Create the label's text
        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto(text_x, text_y),
            Fg(self.theme.get_fg_rgb()),
            &self.text,
            Fg(Reset)
        ).unwrap();
    }
}

/// The main window for the terminal application; this contains all the widgets.
/// Usage examples will appear here as soon as a semi-stable release comes out.
pub struct Window {
    /// The stdout to which all the widgets are printed.
    stdout: RawTerminal<std::io::Stdout>,
    /// The [`themes::Theme`] that the window uses. Can be overridden by theme-changing
    /// builder functions.
    theme: themes::Theme,
    /// All the immediate children of this widget (e.g., excludes grandchildren, 
    /// great-grandchildren, etc.)
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
    fn build(self) -> Window {
        Window { stdout: self.stdout, theme: self.theme, widgets: self.widgets }
    }

    /// Returns a new [`Window`], which can then be built using the builder functions.
    fn builder() -> Window {
        let out = stdout().into_raw_mode().unwrap();
        Window { stdout: out, theme: themes::default(), widgets: Vec::new() }
    }

    /// Returns a new [`Window`] with the default values. Shorthand for
    /// `Window::builder().build();`.
    fn new() -> Window {
        Window::builder().build()
    }
}