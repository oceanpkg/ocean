//! Package licensing.

use std::{
    fmt,
    borrow::Cow,
};
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};

#[doc(inline)]
pub use license::{SpdxLicense, License as KnownLicense, Expr};

/// Any license.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum License<'a> {
    /// A commonly found license listed [here](https://spdx.org/licenses).
    Known(KnownLicense),
    /// A license unknown to Ocean. This is generally treated as an opaque ID.
    Unknown(Cow<'a, str>),
}

impl From<SpdxLicense> for License<'_> {
    #[inline]
    fn from(spdx: SpdxLicense) -> Self {
        License::Known(spdx.into())
    }
}

impl<'a> From<&'a str> for License<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        if let Ok(l) = KnownLicense::parse(s) {
            License::Known(l)
        } else {
            License::Unknown(Cow::Borrowed(s))
        }
    }
}

impl From<String> for License<'_> {
    #[inline]
    fn from(s: String) -> Self {
        if let Ok(l) = KnownLicense::parse(s.as_str()) {
            License::Known(l)
        } else {
            License::Unknown(Cow::Owned(s))
        }
    }
}

impl fmt::Display for License<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            License::Known(known) => known.fmt(f),
            License::Unknown(unknown) => unknown.fmt(f),
        }
    }
}

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = License<'de>;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a license string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Ok(License::owned(v))
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Ok(License::from(v))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Ok(License::owned(v))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for License<'a> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LicenseVisitor)
    }
}

impl Serialize for License<'_> {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.id())
    }
}

impl<'a> License<'a> {
    /// Creates an instance from `s` where any external reference in `s` is not
    /// kept.
    #[inline]
    pub fn owned<S>(s: S) -> Self
        where S: Into<String> + AsRef<str>
    {
        if let Ok(l) = KnownLicense::parse(s.as_ref()) {
            License::Known(l)
        } else {
            License::Unknown(Cow::Owned(s.into()))
        }
    }

    /// Returns the license's identifier by reference.
    #[inline]
    pub fn id(&self) -> &str {
        match self {
            License::Known(l) => l.id(),
            License::Unknown(id) => id,
        }
    }
}
