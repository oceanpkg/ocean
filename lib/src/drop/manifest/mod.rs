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
pub struct Manifest {
    /// The drop's info.
    pub meta: Meta,

    /// The drops that this drop relies on.
    #[serde(rename = "dependencies")]
    pub deps: Option<Deps>,
}

impl Manifest {
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
    pub fn parse_toml(toml: &str) -> Result<Self, toml::de::Error> {
        toml::de::from_str(toml)
    }

    /// Parses a manifest from a [TOML](https://en.wikipedia.org/wiki/TOML) file
    /// at the given path.
    #[cfg(feature = "toml")]
    pub fn read_toml_file<T>(toml: T) -> Result<Self, std::io::Error>
        where T: AsRef<std::path::Path>
    {
        use std::io::{Error, ErrorKind, Read};

        let mut buf = String::with_capacity(128);
        std::fs::File::open(toml)?.read_to_string(&mut buf)?;
        Self::parse_toml(&buf).map_err(|error| {
            Error::new(ErrorKind::InvalidData, error)
        })
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
    pub fn parse_json(json: &str) -> Result<Self, json::Error> {
        json::from_str(json)
    }

    /// Parses a manifest from [JSON](https://en.wikipedia.org/wiki/JSON)
    /// provided by the reader.
    #[cfg(feature = "serde_json")]
    pub fn read_json<J>(json: J) -> Result<Self, json::Error>
        where J: std::io::Read
    {
        json::from_reader(json)
    }

    /// Parses a manifest from a [JSON](https://en.wikipedia.org/wiki/JSON) file
    /// at the given path.
    #[cfg(feature = "serde_json")]
    pub fn read_json_file<J>(json: J) -> Result<Self, std::io::Error>
        where J: AsRef<std::path::Path>
    {
        let reader = std::io::BufReader::new(std::fs::File::open(json)?);
        Self::read_json(reader).map_err(Into::into)
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
