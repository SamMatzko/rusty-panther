//! The module that contains all the widgets used in creating GUIs.

use crate::constants::chars;
use crate::themes;
use crate::traits::*;

use std::io::{stdout, stdin, Write};

use termion;
use termion::color::{Bg, Fg, Reset, Rgb};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::*;

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

    let mut stdout = stdout().into_raw_mode().unwrap();

    // Simply write the color to each row
    for h in 0..height {
        write!(
            stdout,
            "{}{}{}",
            cursor::Goto(x, y + h),
            Bg(bg),
            chars::EMPTY.repeat(width as usize)
        ).unwrap();
        
    stdout.flush().unwrap();
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
impl Label {
    // The builder functions. These can be used to optionally customize options.
    // Be sure to call [`build()`] to finalize the creation.

    /// Sets the border configuration [`bool`]s to `border`.
    pub fn set_border(mut self, border: (bool, bool)) -> Label {
        self.border = border;
        self
    }

    // TODO
    // /// Sets the stdout to `stdout`.
    // pub fn set_stdout(mut self, stdout: RawTerminal<std::io::Stdout>) -> Label {
    //     self.stdout = stdout;
    //     self
    // }

    /// Sets the label's text to `text`.
    pub fn set_text(mut self, text: String) -> Label {
        self.text = text;
        self
    }

    /// Sets the label's theme to `theme`.
    pub fn set_theme(mut self, theme: themes::Theme) -> Label {
        self.theme = theme;
        self
    }

    /// Sets the label's width to `width`.
    pub fn set_width(mut self, width: u16) -> Label {
        self.width = width;
        self
    }
}
impl Buildable for Label {

    fn build(self) -> Label {
        let len: u16 = (self.text.len() as u16)+1;
        Label {
            border: self.border,
            stdout: self.stdout,
            text: self.text,
            theme: self.theme,
            width: len,
        }
    }

    fn builder() -> Label {
        Label {
            border: (true, true),
            stdout: stdout().into_raw_mode().unwrap(),
            text: String::from(""),
            theme: themes::default(),
            width: 10
        }
    }

    fn new() -> Label {
        Label::builder().build()
    }
}
impl Widget for Label {

    fn draw(&mut self, x: u16, y: u16, _width: u16, _height: u16) {

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
                3,
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
                1,
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

        self.stdout.flush().unwrap();
    }
}

/// The main window for the terminal application; this contains all the widgets.
/// Usage examples will appear here as soon as a semi-stable release comes out.
pub struct Window<'a> {
    /// The stdout to which all the widgets are printed.
    stdout: RawTerminal<std::io::Stdout>,
    /// The [`themes::Theme`] that the window uses. Can be overridden by theme-changing
    /// builder functions.
    theme: themes::Theme,
    /// All the immediate children of this widget (e.g., excludes grandchildren, 
    /// great-grandchildren, etc.)
    widgets: Vec<Box<&'a mut dyn Widget>>,
}
impl<'a> Window<'a> {

    /// Quits the window.
    pub fn quit(&mut self) {
        write!(self.stdout, "{}", ToMainScreen).unwrap();
        self.stdout.flush().unwrap();
    }

    /// Run the application; this creates the screen and starts the event listener.
    pub fn run(&mut self) {
        
        // Start the event listener
        for c in stdin().keys() {
            match c.unwrap() {
                Key::Ctrl('c') => {
                    self.quit();
                    return;
                },
                _ => {}
            }
            self.stdout.flush().unwrap();
        }
    }
    
    // The builder functions. These can be used to optionally customize options.
    // Be sure to call [`build()`] to finalize the creation.

    // TODO
    // /// Set the stdout for this window to `stdout`.
    // pub fn set_stdout(mut self, stdout: RawTerminal<std::io::Stdout>) -> Window {
    //     self.stdout = stdout;
    //     self
    // }

    /// Set the theme for the window.
    pub fn set_theme(mut self, theme: themes::Theme) -> Window<'a> {
        self.theme = theme;
        self
    }
}
impl<'a> Buildable for Window<'a> {

    fn build(self) -> Window<'a> {
        Window { stdout: self.stdout, theme: self.theme, widgets: self.widgets }
    }

    fn builder() -> Window<'a> {
        let mut out = stdout().into_raw_mode().unwrap();
        write!(out, "{}", ToAlternateScreen).unwrap();
        out.flush().unwrap();
        Window { stdout: out, theme: themes::default(), widgets: Vec::new() }
    }

    fn new() -> Window<'a> {
        Window::builder().build()
    }
}
impl<'a> Parent<'a> for Window<'a> {
    fn add(&mut self, child: Box<&'a mut dyn Widget>, x: u16, y: u16) {
        self.widgets.push(child);
        self.widgets.last_mut().unwrap().draw(x, y, 0, 0);
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        self.stdout.flush().unwrap();
    }
}