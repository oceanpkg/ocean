//! Meta manifest data.

use serde::Deserialize;

mod deps;
mod meta;

#[cfg(test)]
mod tests;

#[doc(inline)]
pub use self::{
    deps::{Deps, DepInfo},
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
    #[cfg(feature = "toml")]
    pub fn parse_toml<'de: 'a>(toml: &'de str) -> Result<Self, toml::de::Error> {
        toml::de::from_str(toml)
    }

    /// Parses a manifest from [JSON](https://en.wikipedia.org/wiki/JSON).
    ///
    /// ```
    /// use oceanpkg::drop::Manifest;
    ///
    /// let json = r#"{
    ///     "meta": {
    ///         "name": "ocean",
    ///         "description": "Cross-platform package manager",
    ///         "version": "0.1.0",
    ///         "license": "Apache-2.0",
    ///         "authors": ["Nikolai Vazquez", "Alex Farra", "Nicole Zhao"],
    ///         "readme": "README.md",
    ///         "changelog": "CHANGELOG.md",
    ///         "git": "https://github.com/oceanpkg/ocean"
    ///     },
    ///     "dependencies": {
    ///         "wget": "*"
    ///     }
    /// }"#;
    /// let manifest = Manifest::parse_json(json).unwrap();
    /// ```
    #[cfg(feature = "serde_json")]
    pub fn parse_json<'de: 'a>(json: &'de str) -> Result<Self, json::Error> {
        json::from_str(json)
    }

    /// Returns `self` as a TOML string.
    #[cfg(feature = "toml")]
    pub fn to_toml(&self, pretty: bool) -> Result<String, toml::ser::Error> {
        if pretty {
            toml::to_string_pretty(self)
        } else {
            toml::to_string(self)
        }
    }

    /// Returns `self` as a JSON string.
    #[cfg(feature = "serde_json")]
    pub fn to_json(&self, pretty: bool) -> Result<String, json::Error> {
        if pretty {
            json::to_string_pretty(self)
        } else {
            json::to_string(self)
        }
    }
}
