//! Versioning schemes.

use std::borrow::Cow;

pub use semver::Version as SemVer;

/// A drop version.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Version<'a> {
    /// [Semantic versioning](http://semver.org). This is the default.
    #[serde(rename = "semver")]
    SemVer(SemVer),
    /// A custom versioning scheme.
    #[serde(rename = "custom")]
    Custom(Cow<'a, str>),
}

impl<'a> Version<'a> {
    /// Creates a new instance from a custom `version`.
    #[inline]
    pub fn custom<V>(version: V) -> Self
        where V: Into<Cow<'a, str>>
    {
        Self::Custom(version.into())
    }

    /// Attempts to parse `version` as SemVer.
    #[inline]
    pub fn semver(version: &str) -> Result<Self, semver::SemVerError> {
        SemVer::parse(version).map(Self::SemVer)
    }
}
