//! Package licensing.
//!
//! **Note:** this module re-exports all of [`linfo`](https://docs.rs/linfo).

use std::{
    fmt,
    borrow::Cow,
};

mod serde;

// BUG(docs): `Expr` and `SpdxLicense` don't get rendered despite the glob. They
// would, however, still be available within this module with just the glob.
#[doc(inline)]
pub use linfo::{*, Expr, SpdxLicense};

/// Any license, known or otherwise.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AnyLicense<'a> {
    /// A license known to Ocean.
    ///
    /// This means information such as OSI approval and "libre"-ness can be
    /// checked.
    Known(License),
    /// A license unknown to Ocean. This is generally treated as an opaque ID.
    Unknown(Cow<'a, str>),
}

impl From<License> for AnyLicense<'_> {
    #[inline]
    fn from(known: License) -> Self {
        Self::Known(known)
    }
}

impl From<SpdxLicense> for AnyLicense<'_> {
    #[inline]
    fn from(spdx: SpdxLicense) -> Self {
        Self::Known(spdx.into())
    }
}

impl<'a> From<&'a str> for AnyLicense<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        if let Ok(l) = License::parse(s) {
            Self::Known(l)
        } else {
            Self::Unknown(Cow::Borrowed(s))
        }
    }
}

impl From<String> for AnyLicense<'_> {
    #[inline]
    fn from(s: String) -> Self {
        Cow::<'_, str>::Owned(s).into()
    }
}

impl<'a> From<Cow<'a, str>> for AnyLicense<'a> {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        if let Ok(l) = License::parse(s.as_ref()) {
            Self::Known(l)
        } else {
            Self::Unknown(s)
        }
    }
}

impl fmt::Display for AnyLicense<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Known(known) => known.fmt(f),
            Self::Unknown(unknown) => unknown.fmt(f),
        }
    }
}

impl<'a> AnyLicense<'a> {
    /// Creates an instance from `s` where any external reference in `s` is not
    /// kept.
    #[inline]
    pub fn owned<S>(s: S) -> Self
        where S: Into<String> + AsRef<str>
    {
        if let Ok(l) = License::parse(s.as_ref()) {
            Self::Known(l)
        } else {
            Self::Unknown(Cow::Owned(s.into()))
        }
    }

    /// Returns the license's identifier by reference.
    #[inline]
    pub fn id(&self) -> &str {
        match self {
            Self::Known(l) => l.id(),
            Self::Unknown(id) => id,
        }
    }

    /// Returns whether the license is known to Ocean.
    #[inline]
    pub fn is_known(&self) -> bool {
        match self {
            Self::Known(_) => true,
            Self::Unknown(_) => false,
        }
    }
}
