//! The module that contains all the widget structure-related structs. This includes
//! row/column/grid configuration structs and theme structs.

use crate::traits::Buildable;
use crossterm::style::Color;
use crossterm::terminal::size;

#[cfg(test)]
/// The module containing tests for these structs
mod test {
    
    use super::*;

    #[test]
    /// Test the [`Grid`] struct
    fn test_grid() {

        // Create the default grid for testing
        let grid = Grid::builder()
            .width(200)
            .height(50)
            .build();
        
        assert_eq!(grid.get_placement(1, 2), (1, 21));
        assert_eq!(grid.get_placement(2, 3), (21, 41));
        assert_eq!(grid.get_placement(4, 3), (61, 41));

        assert_eq!(grid.get_placement_chars(1, 2), (1, 11));
        assert_eq!(grid.get_placement_chars(2, 3), (41, 21));
        assert_eq!(grid.get_placement_chars(3, 4), (51, 31));
    }
}

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
    height_chars: u16,
    width_: u8,
    width_chars: u16,
}
impl Grid {

    /// Configure the size of a particular column, and set its priority to [`true`]
    pub fn column_configure(&mut self, col: usize, percent: u8) {
        self.columns[col] = GridColumn(percent, true);
        self.recalculate();
    }

    /// Get the size of column `col` in characters, based on the terminal's width
    pub fn get_column_chars(&self, col: u8) -> u16 {
        ((self.columns[col as usize].0 / 100) as u16 * self.width_chars) as u16
    }

    /// Get the placement in % terminal size of the character in the top left
    /// corner of row index `row` and column index `col`
    pub fn get_placement(&self, row: u8, col: u8) -> (u8, u8) {
        
        let mut from_left: u8 = 1;
        let mut from_top: u8 = 1;

        for r in 1..=row {
            println!("Row: {}", r);
            if r > 1 {
                let gridrow = &self.rows[r as usize];
                println!("{:?} gridrow.0: {}", gridrow, gridrow.0);
                from_top += gridrow.0;
            }
        }
        for c in 1..=col {
            println!("Column: {}", c);
            if c > 1 {
                let gridcol = &self.columns[c as usize];
                println!("{:?} gridcol.0: {}", gridcol, gridcol.0);
                from_left += gridcol.0;
            }
        }
        
        (from_top, from_left)
    }

    /// Get the placement in chars of the character in the top left corner of row
    /// index `ro` and column index `col`.
    pub fn get_placement_chars(&self, row: u8, col: u8) -> (u16, u16) {
        let (x, y) = self.get_placement(row, col);
        println!("x, y: {}, {}", x, y);
        (self.percent_to_char(x), self.percent_to_char(y))
    }

    /// Get the size of row `row` in characters, based on the terminal's height
    pub fn get_row_chars(&self, row: u8) -> u16 {
        ((self.rows[row as usize].0 / 100) as u16 * self.height_chars) as u16
    }

    /// Return the number of characters taking up `percent` percent of the screen
    pub fn percent_to_char(&self, percent: u8) -> u16 {
        (percent / 100) as u16 * self.width_chars as u16
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
        println!("percent_for_columns: {}", percent_for_columns);
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

    /// Set the height of the grid in characters. NOT a builder method.
    pub fn set_height_chars(&mut self, size: u16) {
        self.height_chars = size;
    }

    /// Set the width of the grid in charaters. NOT a builder method.
    pub fn set_width_chars(&mut self, size: u16) {
        self.width_chars = size;
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
        Grid {
            columns: self.columns,
            rows: self.rows,
            height_: self.height_,
            height_chars: self.height_chars,
            width_: self.width_,
            width_chars: self.width_chars,
        }
    }

    fn builder() -> Grid {
        let col = GridColumn(20, false);
        let row = GridRow(20, false);
        Grid {
            columns: vec![col.copy(), col.copy(), col.copy(), col.copy(), col.copy()],
            rows: vec![row.copy(), row.copy(), row.copy(), row.copy(), row.copy()],
            height_: 5,
            height_chars: size().expect("size()").1,
            width_: 5,
            width_chars: size().expect("size()").0,
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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
