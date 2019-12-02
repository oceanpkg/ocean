//! Versioning schemes.

use std::borrow::Cow;

#[doc(inline)]
pub use semver::Version as SemVer;

flexible! {
    /// A drop version.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
    pub enum Version<'a> {
        /// [Semantic versioning](http://semver.org). This is the default.
        #[serde(rename = "semver")]
        SemVer(SemVer),
        /// A custom versioning scheme.
        #[serde(rename = "custom")]
        Custom(Cow<'a, str>),
    }
}

impl From<SemVer> for Version<'_> {
    #[inline]
    fn from(v: SemVer) -> Self {
        Version::SemVer(v)
    }
}

impl PartialEq<str> for Version<'_> {
    fn eq(&self, s: &str) -> bool {
        match self {
            // TODO: Switch to a `SemVer` type that supports string equality
            // without doing full parsing
            Self::SemVer(v) => Ok(v) == SemVer::parse(s).as_ref(),
            Self::Custom(v) => v == s,
        }
    }
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
    pub fn parse_semver(version: &str) -> Result<Self, semver::SemVerError> {
        SemVer::parse(version).map(Self::SemVer)
    }

    /// Returns the name of the version kind: `semver` or `custom`.
    #[inline]
    pub fn kind(&self) -> &'static str {
        match self {
            Version::SemVer(_) => "semver",
            Version::Custom(_) => "custom",
        }
    }
}
