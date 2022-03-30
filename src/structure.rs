//! The module that contains all the widget structure-related structs. This includes
//! row/column/grid configuration structs and theme structs.

use crate::traits::Buildable;
use crossterm::style::Color;

/// This struct contains sizing data used in gridding widgets, including how many
/// rows/columns a parent widget has, and how much of the grid those rows/columns
/// take up.
/// 
/// Default width×height is 5×5.
pub struct Grid {
    /// A [`Vec<GridColumn>`] containing all of this grid's columns
    pub columns: Vec<GridColumn>,
    /// A [`Vec<GridRow>`] containing all of this grid's rows
    pub rows: Vec<GridRow>,
    height_: u8,
    width_: u8,
}
impl Grid {

    /// Configure the size of a particular column, and set its priority to [`true`]
    pub fn column_configure(&mut self, col: usize, percent: u8) {
        self.columns[col] = GridColumn(percent, true);
        self.recalculate();
    }

    /// Recalculate the size of all the rows and columns based on which ones have
    /// user-set percentates.
    pub fn recalculate(&mut self) {

        // First calculate the rows, giving prioritized rows the priority

        /* Loop over all the rows, subtracting the prioritized rows' percent from
        the available room percentage, and subtracting `1` from the number of
        total rows. This leaves us with the percent that the unprioritized rows
        will take up (`row_p`), and the number of unprioritized rows there are
        (`rows`)
        */
        let mut row_p = 100;
        let mut rows = self.rows.len();
        for row in &self.rows {
            if row.1 {
                row_p -= row.0;
                rows -= 1;
            }
        }

        // Now go through all the UNprioritized rows and divide the remaing
        // percent up between them
        let percent_for_rows = row_p / rows as u8;
        let mut i = 0;
        for row in &self.rows.clone() {
            if !row.1 {
                self.rows[i] = GridRow(percent_for_rows, false);
            }
            i += 1;
        }

        // Now calculate the columns, giving prioritized columns the priority

        /* Loop over all the columns, subtracting the prioritized columns' percent from
        the available room percentage, and subtracting `1` from the number of
        total columns. This leaves us with the percent that the unprioritized columns
        will take up (`column_p`), and the number of unprioritized columns there are
        (`columns`)
        */
        let mut column_p = 100;
        let mut columns = self.columns.len();
        for column in &self.columns {
            if column.1 {
                column_p -= column.0;
                columns -= 1;
            }
        }

        // Now go through all the UNprioritized columns and divide the remaing
        // percent up between them
        let percent_for_columns = column_p / columns as u8;
        let mut i = 0;
        for column in &self.columns.clone() {
            if !column.1 {
                self.columns[i] = GridColumn(percent_for_columns, false);
            }
            i += 1;
        }
    }

    /// Configure the size of a particular row, and set its priority to [`true`]
    pub fn row_configure(&mut self, row: usize, percent: u8) {
        self.rows[row] = GridRow(percent, true);
        self.recalculate();
    }

    // These methods are the builder-pattern methods; they need to be called in
    // between `builder()` and `build()`
    
    /// Set the height of the grid, in rows
    pub fn height(mut self, height: u8) -> Grid {

        // Set the height
        self.height_ = height;

        // Re-configure the list of rows based on the height given, calculating
        // the new row-size percent
        let percent: u8 = 100 / self.height_;
        self.rows = Vec::new();
        for _ in 0..self.height_ { self.rows.push(GridRow(percent, false)) }
        self
    }

    /// Set the width of the grid, in columns
    pub fn width(mut self, width: u8) -> Grid {

        // Set the width
        self.width_ = width;

        // Re-configure the list of columns based on the width given, calculating
        // the new column-size percent
        let percent: u8 = 100 / self.width_;
        self.columns = Vec::new();
        for _ in 0..self.width_ { self.columns.push(GridColumn(percent, false)) }
        self
    }
}
impl Buildable for Grid {

    fn build(self) -> Grid {
        Grid { columns: self.columns, rows: self.rows, height_: self.height_, width_: self.width_ }
    }

    fn builder() -> Grid {
        let col = GridColumn(20, false);
        let row = GridRow(20, false);
        Grid {
            columns: vec![col.copy(), col.copy(), col.copy(), col.copy(), col.copy()],
            rows: vec![row.copy(), row.copy(), row.copy(), row.copy(), row.copy()],
            height_: 5,
            width_: 5,
        }
    }

    fn new() -> Grid {
        Grid::builder().build()
    }
}

/// The struct for storing a grid column's data.
/// 
/// The [`u8`] is the percentage of the grid's width that this column will take
/// up. The [`bool`] tells whether this column's size should be given a priority
/// or not.
#[derive(Clone)]
pub struct GridColumn(u8, bool);
impl GridColumn {
    /// Return a new [`GridColumn`] with the same configurations as this one
    pub fn copy(&self) -> GridColumn {
        GridColumn(self.0, self.1)
    }
}

/// The struct for storing a grid row's data
/// 
/// The [`u8`] is the percentage of the grid's height that this row will take
/// up. The [`bool`] tells whether this row's size should be given a priority
/// or not.
#[derive(Clone)]
pub struct GridRow(u8, bool);
impl GridRow {
    /// Return a new [`GridRow`] with the same configurations as this one
    pub fn copy(&self) -> GridRow {
        GridRow(self.0, self.1)
    }
}

/// The struct used for creating and setting widget themes.
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
pub fn default_theme() -> Theme {
    Theme::new()
}