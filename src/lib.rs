//! Rusty Panther is an easy-to-use terminal GUI crate, using the Termion low-level
//! terminal manipulation library. Once more code gets written, documentation and
//! examples will appear here.

pub mod theme;
pub mod widgets;

use std::io::Write;

/// The struct that runs and handles everything. Will have one main window, and any
/// amount of sub-windows.
pub struct Application<W: Write> {
    pub window: widgets::Window<W>,
}