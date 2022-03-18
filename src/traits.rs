//! The crate that contains all the traits, for better organization and easier use.

/// The trait that implements all required functions for structs that can be built
/// using the builder pattern syntax.
pub trait Buildable {
    
    /// Returns the completed widget as part of the builder pattern.
    fn build(self) -> Self;

    /// The start of the builder-pattern chain. All other builder methods must be
    /// called on the return result of this one.
    fn builder() -> Self;

    /// Shorthand for `BuildableStruct::builder().build()`.
    fn new() -> Self;
}

/// The trait that implements all required for widget structs.
pub trait Widget {

    /// Draws the widget.
    fn draw(&self);
}