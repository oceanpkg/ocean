//! Ocean packages, also known as drops 💧.

mod app;
mod exe;
mod font;
mod lib;

pub mod name;

pub use self::{
    app::App,
    exe::Exe,
    font::Font,
    lib::Lib,
};

/// Defines an Ocean package, also known as a drop 💧.
#[derive(Clone, Debug)]
pub enum Drop {
    /// A package with a graphical interface.
    App(App),
    /// A package that can be executed; e.g. CLI tool or script.
    Exe(Exe),
    /// A package for a typeface with specific properties; e.g. bold, italic.
    Font(Font),
    /// A package for a library of a given language.
    Lib(Lib),
}

impl From<App> for Drop {
    #[inline]
    fn from(drop: App) -> Self {
        Self::App(drop)
    }
}

impl From<Exe> for Drop {
    #[inline]
    fn from(drop: Exe) -> Self {
        Self::Exe(drop)
    }
}

impl From<Font> for Drop {
    #[inline]
    fn from(drop: Font) -> Self {
        Self::Font(drop)
    }
}

impl From<Lib> for Drop {
    #[inline]
    fn from(drop: Lib) -> Self {
        Self::Lib(drop)
    }
}

impl Drop {
    ///
    pub fn query(query: &name::DropQuery) -> Result<Self, ()> {
        unimplemented!("TODO: Find '{}' drop", query);
    }

    /// Returns basic metadata for the drop.
    pub fn metadata(&self) -> &Metadata {
        match self {
            Drop::App(app)   => app.metadata(),
            Drop::Exe(exe)   => exe.metadata(),
            Drop::Font(font) => font.metadata(),
            Drop::Lib(lib)   => lib.metadata(),
        }
    }
}

/// Information common to all drops.
#[derive(Clone, Debug)]
pub struct Metadata {
    // TODO: Replace `scope` and `name` with a `ScopedName`

    /// The drop's namespace.
    pub scope: String,
    /// The drop's unique name within its namespace.
    pub name: String,
}
