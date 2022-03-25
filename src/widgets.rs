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
/// 
/// Example:
/// 
/// ```ignore
/// use rusty_panther::prelude::*;
/// use rusty_panther::widgets::*;
/// 
/// fn main() {
/// 
///     // Create the window
///     let mut window = Window::new();
///
///     // Create the label
///     let mut label = Label::builder()
///         .text(String::from("This is text right here."))
///         .build();
///     window.add(Box::new(&mut label), 1, 1);
///     window.run();
/// }
/// ```
pub struct Label {
    /// A tuple containg two [`bool`]s; whether there is a border, and whether to
    /// show the border
    border_: (bool, bool),
    /// The stdout to which all the widgets are printed (not very effective at the
    /// moment; there's no guarantee that all widgets will be printed to this stdout)
    stdout: RawTerminal<std::io::Stdout>,
    /// The text that the label contains
    text_: String,
    /// The [`themes::Theme`] that this label uses for it's colors
    theme_: themes::Theme,
    /// The width of the label, in chars
    width: u16,
}
impl Label {
    // The builder functions. These can be used to optionally customize options.
    // Be sure to call [`build()`] to finalize the creation.

    /// Sets the border configuration [`bool`]s to `border`. `border` is tuple
    /// containg two [`bool`]s; whether there is a border, and whether to show
    /// the border. Use when building the label.
    /// 
    /// For example:
    /// 
    /// ```ignore
    /// let label = Label::builder()
    ///     .border((true, false))
    ///     .build();
    /// ```
    pub fn border(mut self, border: (bool, bool)) -> Label {
        self.border_ = border;
        self
    }

    // TODO
    // /// Sets the stdout to `stdout`.
    // pub fn set_stdout(mut self, stdout: RawTerminal<std::io::Stdout>) -> Label {
    //     self.stdout = stdout;
    //     self
    // }

    /// Sets the label's text to `text`, a [`String`]. Use when building the label.
    /// 
    /// For example:
    /// 
    /// ```ignore
    /// let label = Label::builder()
    ///     .text(String::from("This is text."))
    ///     .build();
    /// ```
    pub fn text(mut self, text: String) -> Label {
        self.text_ = text;
        self
    }

    /// Sets the label's theme to `theme`, a [`themes::Theme`]. Use when building the label.
    /// 
    /// For example:
    /// 
    /// ```ignore
    /// let label = Label::builder()
    ///     .theme(themes::default())
    ///     .build();
    /// ```
    pub fn set_theme(mut self, theme: themes::Theme) -> Label {
        self.theme_ = theme;
        self
    }

    /// Sets the label's width to `width`, a [`u16`]. Use when building the label.
    /// 
    /// For example:
    /// 
    /// ```ignore
    /// let label = Label::builder()
    ///     .width(3)
    ///     .build();
    /// ```
    pub fn set_width(mut self, width: u16) -> Label {
        self.width = width;
        self
    }
}
impl Buildable for Label {

    fn build(self) -> Label {
        let len: u16 = (self.text_.len() as u16)+1;
        Label {
            border_: self.border_,
            stdout: self.stdout,
            text_: self.text_,
            theme_: self.theme_,
            width: len,
        }
    }

    fn builder() -> Label {
        Label {
            border_: (true, true),
            stdout: stdout().into_raw_mode().unwrap(),
            text_: String::from(""),
            theme_: themes::default(),
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
        if self.border_.0 {

            // Make sure the text doesn't end up on the border
            text_x += 1;
            text_y += 1;

            // Create the bordered box
            create_border_box(
                x,
                y,
                self.width + 1,
                3,
                self.theme_.get_fg_rgb(),
                self.theme_.get_bg_rgb()
            );
        }
        else {
            
            // Create the unbordered box
            create_fill_box(
                x,
                y,
                self.width,
                1,
                self.theme_.get_bg_rgb()
            );
        }

        // Create the label's text
        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto(text_x, text_y),
            Fg(self.theme_.get_fg_rgb()),
            &self.text_,
            Fg(Reset)
        ).unwrap();

        self.stdout.flush().unwrap();
    }
}

/// The main window for the terminal application; this contains all the widgets.
/// Usage examples will appear here as soon as a semi-stable release comes out.
/// 
/// Example:
/// 
/// ```ignore
/// use rusty_panther::prelude::*;
/// 
/// fn main() {
///     
///     // The window
///     let mut window = widgets::Window:new();
///     window.run();
/// }
/// ```
pub struct Window<'a> {
    /// All the immediate children of this widget (e.g., excludes grandchildren, 
    /// great-grandchildren, etc.)
    children: Vec<Box<&'a mut dyn Widget>>,
    /// The stdout to which all the widgets are printed.
    stdout: RawTerminal<std::io::Stdout>,
    /// The [`themes::Theme`] that the window uses.
    theme_: themes::Theme,
}
impl<'a> Window<'a> {

    /// Quits the window and the alternate screen.
    pub fn quit(&mut self) {
        write!(self.stdout, "{}", ToMainScreen).unwrap();
        self.stdout.flush().unwrap();
    }

    /// Run the application; this creates the screen and starts the event listener.
    /// 
    /// More information on connection to events will appear here when implemented.
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

    /// Set the theme for the window. Use when building the window.
    /// 
    /// For example:
    /// 
    /// ```ignore
    /// let window = Window::builder()
    ///     .theme(themes::default())
    ///     .build();
    /// ```
    pub fn theme(mut self, theme: themes::Theme) -> Window<'a> {
        self.theme_ = theme;
        self
    }
}
impl<'a> Buildable for Window<'a> {

    fn build(self) -> Window<'a> {
        Window { children: self.children, stdout: self.stdout, theme_: self.theme_ }
    }

    fn builder() -> Window<'a> {
        let mut out = stdout().into_raw_mode().unwrap();
        write!(out, "{}", ToAlternateScreen).unwrap();
        out.flush().unwrap();
        Window { children: Vec::new(), stdout: out, theme_: themes::default() }
    }

    fn new() -> Window<'a> {
        Window::builder().build()
    }
}
impl<'a> Parent<'a> for Window<'a> {
    fn add(&mut self, child: Box<&'a mut dyn Widget>, x: u16, y: u16) {
        self.children.push(child);
        self.children.last_mut().unwrap().draw(x, y, 0, 0);
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        self.stdout.flush().unwrap();
    }
}