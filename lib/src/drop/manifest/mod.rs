//! Meta manifest data.

use std::{
    collections::BTreeMap,
    fmt,
};
use serde::Deserialize;
use crate::drop::name::ValidName;

mod dep;
mod meta;
mod version;
pub mod git;

#[cfg(test)]
mod tests;

#[doc(inline)]
pub use self::{
    dep::Dep,
    git::Git,
    meta::Meta,
    version::Version,
};

/// A drop manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Manifest<'a> {
    /// The drop's info.
    #[serde(borrow)]
    pub meta: Meta<'a>,

    /// What does this drop rely on?
    #[serde(rename = "dependencies")]
    pub deps: Option<BTreeMap<&'a ValidName, Dep<'a>>>,
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
    ///     readme = "README.md"
    ///     changelog = "CHANGELOG.md"
    ///     git = "https://github.com/oceanpkg/ocean"
    ///
    ///     [dependencies]
    ///     wget = "*"
    /// "#;
    /// let manifest = Manifest::parse_toml(toml).unwrap();
    /// let meta = &manifest.meta;
    ///
    /// assert_eq!(meta.name, "ocean");
    /// assert_eq!(meta.description, "Cross-platform package manager");
    /// assert_eq!(&meta.version, "0.1.0");
    /// assert_eq!(meta.license.as_ref().unwrap(), "Apache-2.0");
    /// assert_eq!(meta.readme.unwrap(), "README.md");
    /// assert_eq!(meta.changelog.unwrap(), "CHANGELOG.md");
    /// assert_eq!(meta.git.as_ref().unwrap().repo(), "https://github.com/oceanpkg/ocean");
    ///
    /// for (name, info) in manifest.deps.unwrap() {
    ///     assert_eq!(name, "wget");
    ///     assert_eq!(info.version(), "*");
    /// }
    /// ```
    pub fn parse_toml(toml: &'a str) -> Result<Manifest<'a>, toml::de::Error> {
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
