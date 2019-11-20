//! Drop manifest data.

use std::borrow::Cow;
use serde::Deserialize;
use crate::drop::{license::Expr, name::ValidName, Version};

/// A drop manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Manifest<'a> {
    /// The drop's name.
    pub name: &'a ValidName,

    /// What is this drop?
    pub description: &'a str,

    /// The drop version.
    pub version: Version<'a>,

    /// The license used.
    pub license: Option<Expr<'a>>,

    /// Authors of the drop.
    pub authors: Option<Vec<&'a str>>,

    /// A path to the package's "README" file.
    pub readme: Option<&'a str>,

    /// A path to the package's change log file.
    pub changelog: Option<&'a str>,
}

impl<'a> Manifest<'a> {
    /// A dummy manifest for example purposes.
    pub const DUMMY: Self = Manifest {
        name: ValidName::OCEAN,
        description: env!("CARGO_PKG_DESCRIPTION"),
        version: Version::Custom(Cow::Borrowed(env!("CARGO_PKG_VERSION"))),
        license: None,
        authors: None,
        readme: Some("README.md"),
        changelog: Some("CHANGELOG.md"),
    };

    /// Parses a manifest from [TOML](https://en.wikipedia.org/wiki/TOML).
    ///
    /// ```
    /// use oceanpkg::drop::{
    ///     Manifest,
    ///     name::ValidName,
    ///     Version,
    ///     license::SpdxLicense,
    /// };
    ///
    /// let manifest = Manifest {
    ///     name: ValidName::OCEAN,
    ///     description: "Cross-platform package manager",
    ///     version: Version::custom("0.1"),
    ///     license: Some(SpdxLicense::Apache2.into()),
    ///     authors: None,
    ///     readme: Some("README.md"),
    ///     changelog: Some("CHANGELOG.md"),
    /// };
    ///
    /// let toml = r#"
    /// name = "ocean"
    /// description = "Cross-platform package manager"
    /// version = { custom = "0.1" }
    /// license = "Apache-2.0"
    /// readme = "README.md"
    /// changelog = "CHANGELOG.md"
    /// "#;
    ///
    /// assert_eq!(Manifest::from_toml(toml), Ok(manifest));
    /// ```
    #[inline]
    pub fn from_toml(toml: &'a str) -> Result<Manifest<'a>, toml::de::Error> {
        toml::de::from_str(toml)
    }
}
