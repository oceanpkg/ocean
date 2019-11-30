//! Client library for the [Ocean package manager][home].
//!
//! This library is meant to be used by Ocean's [CLI], [GUI], and web services.
//!
//! [home]: https://www.oceanpkg.org
//! [CLI]: https://en.wikipedia.org/wiki/Command-line_interface
//! [GUI]: https://en.wikipedia.org/wiki/Graphical_user_interface

#![deny(missing_docs)]

#![doc(html_root_url = "https://docs.rs/oceanpkg/0.0.8")]
#![doc(html_logo_url = "https://www.oceanpkg.org/static/images/ocean-logo.svg")]

#[macro_use] extern crate serde;
#[macro_use] extern crate static_assertions;

#[macro_use]
pub(crate) mod flexible;

pub mod cfg;
pub mod drop;
pub mod ext;
pub mod install;

#[doc(inline)]
pub use self::drop::Drop;
