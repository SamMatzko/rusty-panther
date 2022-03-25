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

/// The trait for any widgets that are parents; that is, they contain child widgets.
pub trait Parent<'a> {

    /// Adds a child widget to this parent widget. Takes arguments specifying what
    /// percetage of the window's width and height this widget should take up; these
    /// percentages are NOT checked for validity. This means that it's possible to
    /// have a widget that takes up 110% of the available space. May fix this in
    /// the future.
    fn add(&mut self, child: Box<&'a mut dyn Widget>, width: u16, height: u16);
}

/// The trait for widget structs.
pub trait Widget {

    /// Draws the widget, with parameters location (`x`, `y`) and size `width`×`height`.
    /// This function is called by the parent widgets.
    fn draw(&mut self, x: u16, y: u16, width: u16, height: u16);
}