//! Versioning schemes.

use std::fmt;
use serde::{Serialize, Serializer};

#[doc(inline)]
pub use semver::Version as SemVer;

flexible! {
    /// A drop version.
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Version {
        /// [Semantic versioning](http://semver.org). This is the default.
        #[serde(rename = "semver")]
        SemVer(SemVer),
        /// A custom versioning scheme.
        #[serde(rename = "custom")]
        Custom(String),
    }
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

impl fmt::Display for Version {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SemVer(v) => v.fmt(f),
            Self::Custom(v) => v.fmt(f),
        }
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeMap;

        match self {
            Version::SemVer(semver) => ser.collect_str(semver),
            Version::Custom(custom) => {
                let mut map = ser.serialize_map(Some(1))?;
                map.serialize_entry("custom", custom)?;
                map.end()
            },
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

    /// Returns the name of the version kind: `semver` or `custom`.
    #[inline]
    pub fn kind(&self) -> &'static str {
        match self {
            Version::SemVer(_) => "semver",
            Version::Custom(_) => "custom",
        }
    }
}
