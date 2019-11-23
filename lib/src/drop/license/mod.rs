//! Package licensing.

use std::{
    fmt,
    borrow::Cow,
};

mod serde;

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

impl From<KnownLicense> for License<'_> {
    #[inline]
    fn from(known: KnownLicense) -> Self {
        Self::Known(known)
    }
}

impl From<SpdxLicense> for License<'_> {
    #[inline]
    fn from(spdx: SpdxLicense) -> Self {
        Self::Known(spdx.into())
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
