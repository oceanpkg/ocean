//! Version 1 of Ocean's web API.

/// The default URL to which API calls are made: `https://api.oceanpkg.org/v1/`.
pub const DEFAULT_URL: &str = "https://api.oceanpkg.org/v1/";

#[cfg(feature = "reqwest")]
mod login;

#[cfg(feature = "reqwest")]
mod ship;

#[cfg(feature = "reqwest")]
#[doc(inline)]
pub use self::{
    login::*,
    ship::*,
};
