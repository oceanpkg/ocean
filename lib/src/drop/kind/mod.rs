//! Drop kinds.

mod app;
mod exe;
mod font;
mod lib;

#[doc(inline)]
pub use self::{
    app::App,
    exe::Exe,
    font::Font,
    lib::Lib,
};

/// The type of package a drop can be.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    /// Has a graphical interface.
    App,
    /// Can be executed; e.g. CLI tool or script.
    Exe,
    /// A typeface with specific properties; e.g. bold, italic.
    Font,
    /// A library of a given language.
    Lib,
}
