//! Rusty Panther is an easy-to-use terminal GUI crate, using the Termion low-level
//! terminal manipulation library. Once more code gets written, documentation and
//! examples will appear here.

pub mod constants;
pub mod structure;
pub mod traits;
pub mod widgets;

/// The `prelude` module for `rusty_panther`; contains all the necessary traits.
pub mod prelude {
    pub use crate::structure::*;
    pub use crate::traits::*;
    pub use crate::widgets;
}
