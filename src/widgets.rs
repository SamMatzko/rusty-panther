//! The module that contains all the widgets used in creating GUIs.

use crate::constants::chars;
use crate::structure::*;
use crate::traits::*;

use crossterm::{cursor, execute};
use crossterm::event::*;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::*;

use std::io::{stdout, Write};

/// A function that creates a border box
fn create_border_box(x: u16, y: u16, width: u16, height: u16, fg: Color, bg: Color) {
    
    let mut stdout = stdout();

    // Create the top of the box
    execute!(
        stdout,
        cursor::MoveTo(x, y),
        SetForegroundColor(fg),
        SetBackgroundColor(bg),
        Print(chars::TOP_LEFT),
        Print(chars::HORIZONTAL.repeat((width - 2) as usize)),
        Print(chars::TOP_RIGHT),
        ResetColor
    ).unwrap();

    // Create all the sides
    for i in 0..(height - 2) {
        execute!(
            stdout,
            cursor::MoveTo(x, y + (i + 1)),
            SetForegroundColor(fg),
            SetBackgroundColor(bg),
            Print(chars::VERTICAL),
            Print(chars::EMPTY.repeat((width - 2) as usize)),
            Print(chars::VERTICAL),
            ResetColor
        ).unwrap();
    }

    // Create the bottom of the box
    execute!(
        stdout,
        cursor::MoveTo(x, y + height - 1),
        SetForegroundColor(fg),
        SetBackgroundColor(bg),
        Print(chars::BOTTOM_LEFT),
        Print(chars::HORIZONTAL.repeat((width - 2) as usize)),
        Print(chars::BOTTOM_RIGHT),
        ResetColor
    ).unwrap();
}

/// A function that creates a filled, borderless box
fn create_fill_box(x: u16, y: u16, width: u16, height: u16, bg: Color) {

    let mut stdout = stdout();

    // Simply write the color to each row
    for h in 0..height {
        execute!(
            stdout,
            cursor::MoveTo(x, y + h),
            SetBackgroundColor(bg),
            Print(chars::EMPTY.repeat(width as usize))
        ).unwrap();
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
    stdout: std::io::Stdout,
    /// The text that the label contains
    text_: String,
    /// The [`Theme`] that this label uses for it's colors
    theme_: Theme,
    /// The width of the label, in chars
    width: u16,
    /// The x position of this child, in either characters or grid units
    x: u16,
    /// The y position of this child, in either characters or grid units
    y: u16,
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

    /// Sets the label's theme to `theme`, a [`Theme`]. Use when building the label.
    /// 
    /// For example:
    /// 
    /// ```ignore
    /// let label = Label::builder()
    ///     .theme(themes::default())
    ///     .build();
    /// ```
    pub fn set_theme(mut self, theme: Theme) -> Label {
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
            x: self.x,
            y: self.y,
        }
    }

    fn builder() -> Label {
        Label {
            border_: (true, true),
            stdout: stdout(),
            text_: String::from(""),
            theme_: default_theme(),
            width: 10,
            x: 1,
            y: 1,
        }
    }

    fn new() -> Label {
        Label::builder().build()
    }
}
impl Widget for Label {

    fn draw(&mut self, x: u16, y: u16, width: u16, height: u16) {

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
                width + 1,
                height,
                self.theme_.get_fg_rgb(),
                self.theme_.get_bg_rgb()
            );
        }
        else {
            
            // Create the unbordered box
            create_fill_box(
                x,
                y,
                width,
                height,
                self.theme_.get_bg_rgb()
            );
        }

        // Create the label's text
        execute!(
            self.stdout,
            cursor::MoveTo(text_x, text_y),
            SetForegroundColor(self.theme_.get_fg_rgb()),
            Print(&self.text_),
            ResetColor
        ).unwrap();
    }
    fn get_x(&self) -> u16 { self.x }
    fn get_y(&self) -> u16 { self.y }
    fn set_x(&mut self, x: u16) { self.x = x; }
    fn set_y(&mut self, y: u16) { self.y = y; }
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
    /// The [`Grid`] that manages all of the widget-sizing calculations
    grid: Grid,
    /// The height of the terminal screen
    screen_height: u16,
    /// The current width of the terminal screen
    screen_width: u16,
    /// The stdout to which all the widgets are printed.
    stdout: std::io::Stdout,
    /// The [`Theme`] that the window uses.
    theme_: Theme,
}
impl<'a> Window<'a> {

    /// Draws all the child widgets based on the terminal's width and height
    pub fn draw_children(&mut self) {

        // Clear the screen
        execute!(self.stdout, Clear(ClearType::All));

        // Update the grid's size
        self.update_grid_size();

        // For each child widget, calculate its positioning and size
        for child in &mut self.children {
            
            // Get the placement and size of the child
            let (x, y) = self.grid.get_placement_chars(child.get_x() as u8, child.get_y() as u8);
            let width = self.grid.get_column_chars(child.get_x() as u8);
            let height = self.grid.get_row_chars(child.get_y() as u8);
            println!("x×y {}×{}", x, y);
            println!("wxh {}x{}", size().unwrap().0, size().unwrap().1);

            // Place the child
            child.draw(x, y, width, height);
        }
    }

    /// Quits the window and the alternate screen.
    pub fn quit(&mut self) {
        execute!(self.stdout, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }

    /// Run the application; this starts the event listener.
    /// 
    /// More information on connection to events will appear here when implemented.
    pub fn run(&mut self) {

        // Start the event listener
        loop {
            match read().unwrap() {
                Event::Key(
                    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL }
                ) => {
                    self.quit();
                    return;
                },
                Event::Key(_) => {},
                Event::Mouse(_) => {}
                Event::Resize(width, height) => {
                    self.screen_height = height;
                    self.screen_width = width;
                    self.draw_children();
                }
            }
        }
    }

    /// Updates the grid size based on the terminal size.
    fn update_grid_size(&mut self) {
        let (width, height) = size().expect("size()");
        self.grid.set_height_chars(height);
        self.grid.set_width_chars(width);
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
    pub fn theme(mut self, theme: Theme) -> Window<'a> {
        self.theme_ = theme;
        self
    }
}
impl<'a> Buildable for Window<'a> {

    fn build(self) -> Window<'a> {
        Window {
            children: self.children,
            grid: self.grid,
            screen_height: self.screen_height,
            screen_width: self.screen_width,
            stdout: self.stdout,
            theme_: self.theme_
        }
    }

    fn builder() -> Window<'a> {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen).unwrap();
        Window {
            children: Vec::new(),
            grid: Grid::new(),
            screen_height: size().expect("screen size").1,
            screen_width: size().expect("screen size").0,
            stdout: stdout(),
            theme_: default_theme()
        }
    }

    fn new() -> Window<'a> {
        Window::builder().build()
    }
}
impl<'a> Parent<'a> for Window<'a> {
    fn add(&mut self, child: Box<&'a mut dyn Widget>, x: u16, y: u16) {
        self.children.push(child);
        self.children.last_mut().unwrap().draw(x, y, 0, 0);
        execute!(self.stdout, cursor::MoveTo(1, 1)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn grid(&mut self, child: Box<&'a mut dyn Widget>,
        row: u16,
        col: u16,
        rowspan: u16,
        colspan: u16) {

        // Set this new child's row and column
        child.set_x(col);
        child.set_y(row);
        self.children.push(child);
        
        // Redraw the children
        self.draw_children();
    }
}
