//! Crate constants, like characters used in creating different widgets.

#[allow(non_camel_case_types)]
pub struct chars {}
impl chars {
    pub const TOP_LEFT: &'static str = "┌";
    pub const TOP_RIGHT: &'static str = "┐";
    pub const BOTTOM_LEFT: &'static str = "└";
    pub const BOTTOM_RIGHT: &'static str = "┘";
    pub const VERTICAL: &'static str = "│";
    pub const HORIZONTAL: &'static str = "─";
    pub const EMPTY: &'static str = " ";
}