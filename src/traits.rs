//! The crate that contains all the traits, for better organization and easier use.

/// The trait for all structs that can be built using the builder pattern syntax.
pub trait Buildable {
    
    /// Returns the completed widget as part of the builder pattern.
    fn build(self) -> Self;

    /// The start of the builder-pattern chain. All other builder methods must be
    /// called on the return result of this one.
    fn builder() -> Self;

    /// Shorthand for `BuildableStruct::builder().build()`.
    fn new() -> Self;
}

/// The trait for all widgets that can have parents
pub trait Child {
   /// Get the `x` postition of the child, either in characters or in grid units
   fn get_x() -> u16;

   /// Get the `y` position of the child, either in characters or in grid units
   fn get_y() -> u16;
}

/// The trait for any widgets that are parents; that is, they contain child widgets.
pub trait Parent<'a> {

    /// Adds a child widget to this parent widget. Takes placement X and placement
    /// Y arguments. Note that this doesn't affect widgets added by `grid()`,
    /// and hence widgets added with `add()` don't get scaled by the resizing of
    /// the terminal.
    fn add(&mut self, child: Box<&'a mut dyn Widget>, x: u16, y: u16);

    /// Adds a child widget to this parent widget. Takes row, column, rowspan, and
    /// columnspan arguments for where and how to place this widget.
    fn grid(
        &mut self, child: Box<&'a mut dyn Widget>,
        row: u16,
        col: u16,
        rowspan: u16,
        colspan: u16
    );
}

/// The trait for widget structs.
pub trait Widget {

    /// Draws the widget, with parameters location (`x`, `y`) and size `width`×`height`.
    /// This function is called by the parent widgets.
    fn draw(&mut self, x: u16, y: u16, width: u16, height: u16);
}
