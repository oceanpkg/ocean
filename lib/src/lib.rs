//! Client library for the [Ocean package manager](https://www.oceanpkg.org).
//!
//! [home]: https://www.oceanpkg.org

#![deny(missing_docs)]

#[macro_use]
extern crate serde;

pub mod cfg;
pub mod drop;
pub mod ext;
pub mod install;
