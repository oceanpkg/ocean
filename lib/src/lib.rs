//! Client library for the [Ocean package manager][home].
//!
//! This library is meant to be used by Ocean's [CLI], [GUI], and web services.
//!
//! [home]: https://www.oceanpkg.org
//! [CLI]: https://en.wikipedia.org/wiki/Command-line_interface
//! [GUI]: https://en.wikipedia.org/wiki/Graphical_user_interface

#![deny(missing_docs)]

#![doc(html_root_url = "https://docs.rs/oceanpkg/0.0.10")]
#![doc(html_logo_url = "https://www.oceanpkg.org/static/images/ocean-logo.svg")]

extern crate oceanpkg_shared as shared;

#[macro_use] extern crate cfg_if;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde;
#[macro_use] extern crate static_assertions;

#[cfg(feature = "serde_json")]
extern crate serde_json as json;

#[macro_use]
pub(crate) mod flexible;

pub mod api;
pub mod auth;
pub mod cfg;
pub mod drop;
pub mod env;
pub mod install;
pub mod system;

#[doc(inline)]
pub use self::drop::Drop;
