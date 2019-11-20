use semver::Version as SemVer;
use crate::drop;

/// A version field that can either be a SemVer string literal (default) or map
/// defining a specific version type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Version<'a> {
    /// The default version format as a string literal.
    Simple(SemVer),
    /// A specific format choice, e.g. `{ semver = "0.1.0" }` in TOML.
    Detailed(drop::Version<'a>),
}

impl<'a> From<drop::Version<'a>> for Version<'a> {
    #[inline]
    fn from(v: drop::Version<'a>) -> Self {
        match v {
            drop::Version::SemVer(v) => Version::Simple(v),
            other => Version::Detailed(other),
        }
    }
}

impl<'a> From<Version<'a>> for drop::Version<'a> {
    #[inline]
    fn from(v: Version<'a>) -> Self {
        v.into_standard()
    }
}

impl PartialEq<str> for Version<'_> {
    fn eq(&self, s: &str) -> bool {
        match self {
            Version::Simple(v) |
            Version::Detailed(drop::Version::SemVer(v)) => {
                Ok(v) == SemVer::parse(s).as_ref()
            },
            Version::Detailed(drop::Version::Custom(v)) => {
                v == s
            },
        }
    }
}

impl<'a> Version<'a> {
    /// Attempts to parse `version` as SemVer.
    #[inline]
    pub fn parse_semver(version: &str) -> Result<Self, semver::SemVerError> {
        SemVer::parse(version).map(Self::Simple)
    }

    /// Converts `version` into a `Detailed` variant.
    #[inline]
    pub fn choice<V: Into<drop::Version<'a>>>(version: V) -> Self {
        Self::Detailed(version.into())
    }

    /// Normalizes `self` into its simplest form.
    ///
    /// This usually means converting `semver` choice to a simple literal.
    #[inline]
    pub fn normalized(self) -> Self {
        match self {
            Version::Detailed(drop::Version::SemVer(semver)) => {
                Self::Simple(semver)
            },
            other => other,
        }
    }

    /// Converts `self` into a normal `Version` type used elsewhere.
    #[inline]
    pub fn into_standard(self) -> drop::Version<'a> {
        match self {
            Version::Simple(semver) => semver.into(),
            Version::Detailed(v) => v,
        }
    }
}
