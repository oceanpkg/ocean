//! Meta manifest data.

use std::fmt;
use serde::Deserialize;

mod deps;
mod meta;
mod version;
pub mod detailed;
pub mod git;

#[cfg(test)]
mod tests;

use self::detailed::{Detailed, Flexible};

#[doc(inline)]
pub use self::{
    deps::{Deps, DepInfo},
    git::Git,
    meta::Meta,
};

/// A drop manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Manifest<'a> {
    /// The drop's info.
    #[serde(borrow)]
    pub meta: Meta<'a>,

    /// The drops that this drop relies on.
    #[serde(rename = "dependencies")]
    pub deps: Option<Deps<'a>>,
}

impl fmt::Display for Manifest<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_toml(true)
            .map_err(|_| fmt::Error)
            .and_then(|s| s.fmt(f))
    }
}

impl<'a> Manifest<'a> {
    /// Parses a manifest from [TOML](https://en.wikipedia.org/wiki/TOML).
    ///
    /// ```
    /// use oceanpkg::drop::Manifest;
    ///
    /// let toml = r#"
    ///     [meta]
    ///     name = "ocean"
    ///     description = "Cross-platform package manager"
    ///     version = "0.1.0"
    ///     license = "Apache-2.0"
    ///     authors = ["Nikolai Vazquez", "Alex Farra", "Nicole Zhao"]
    ///     readme = "README.md"
    ///     changelog = "CHANGELOG.md"
    ///     git = "https://github.com/oceanpkg/ocean"
    ///
    ///     [dependencies]
    ///     wget = "*"
    /// "#;
    /// let manifest = Manifest::parse_toml(toml).unwrap();
    /// ```
    pub fn parse_toml<'de: 'a>(toml: &'de str) -> Result<Self, toml::de::Error> {
        toml::de::from_str(toml)
    }

    /// Returns `self` as a TOML string.
    pub fn to_toml(&self, pretty: bool) -> Result<String, toml::ser::Error> {
        if pretty {
            toml::to_string_pretty(self)
        } else {
            toml::to_string(self)
        }
    }
}
