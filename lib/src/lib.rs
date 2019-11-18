//! Client library for the [Ocean package manager][home].
//!
//! This library is meant to be used by Ocean's [CLI], [GUI], and web services.
//!
//! [home]: https://www.oceanpkg.org
//! [CLI]: https://en.wikipedia.org/wiki/Command-line_interface
//! [GUI]: https://en.wikipedia.org/wiki/Graphical_user_interface

#![deny(missing_docs)]

#[macro_use] extern crate phf;
#[macro_use] extern crate serde;

#[macro_use]
mod macros;

pub mod cfg;
pub mod drop;
pub mod ext;
pub mod install;
pub mod version;
