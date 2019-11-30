//! Versioning schemes.

#[doc(inline)]
pub use semver::Version as SemVer;

/// A drop version.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Version {
    /// [Semantic versioning](http://semver.org). This is the default.
    #[serde(rename = "semver")]
    SemVer(SemVer),
    /// A custom versioning scheme.
    #[serde(rename = "custom")]
    Custom(String),
}

impl From<SemVer> for Version {
    #[inline]
    fn from(v: SemVer) -> Self {
        Version::SemVer(v)
    }
}

impl PartialEq<str> for Version {
    fn eq(&self, s: &str) -> bool {
        match self {
            // TODO: Switch to a `SemVer` type that supports string equality
            // without doing full parsing
            Self::SemVer(v) => Ok(v) == SemVer::parse(s).as_ref(),
            Self::Custom(v) => v == s,
        }
    }
}

impl Version {
    /// Creates a new instance from a custom `version`.
    #[inline]
    pub fn custom<V>(version: V) -> Self
        where V: Into<String>
    {
        Self::Custom(version.into())
    }

    /// Attempts to parse `version` as SemVer.
    #[inline]
    pub fn parse_semver(version: &str) -> Result<Self, semver::SemVerError> {
        SemVer::parse(version).map(Self::SemVer)
    }
}
