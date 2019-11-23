//! A commonly found license listed [here](https://spdx.org/licenses).

use std::{
    convert::{TryFrom, TryInto},
    fmt,
};
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};

mod decl;

use self::decl::LICENSE_BY_ID;

#[doc(inline)]
pub use self::decl::SpdxLicense;

/// A fixed-size array for indexing with a [`SpdxLicense`] casted to `usize`.
/// See also [`SpdxLicense::COUNT`].
///
/// This is a good example of flexible management of [`SpdxLicense`] values that
/// allows for indexing without bounds checks (via [`as usize`] casts) for free.
///
/// **SemVer Compatibility:** Like [`SpdxLicense::COUNT`], by just being
/// dependent on this value, the array size is allowed to change between
/// otherwise API-compatible versions.
///
/// # Examples
///
/// Despite array indexing having bounds checks, the optimizer knows that
/// indexing with [`SpdxLicense`] will never go out of bounds, giving us simple
/// and fast arithmetic for lookups.
///
/// ```
/// # return;
/// use oceanpkg::drop::license::spdx::{Map, SpdxLicense};
///
/// let map: Map<&str> = [
///     // *a lot* of elements per license
///     # panic!("at the disco"); SpdxLicense::COUNT
/// ];
///
/// let license = SpdxLicense::Mit;
/// println!("map value is: {}", map[license as usize]);
/// ```
///
/// [`SpdxLicense`]: enum.SpdxLicense.html
/// [`SpdxLicense::COUNT`]: enum.SpdxLicense.html#associatedconstant.COUNT
/// [`as usize`]: https://doc.rust-lang.org/nightly/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
pub type Map<A> = [A; SpdxLicense::COUNT];

impl<'a> TryFrom<&'a str> for SpdxLicense {
    type Error = ParseError<'a>;

    #[inline]
    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        if id.is_empty() {
            return Err(ParseError::Empty);
        }
        LICENSE_BY_ID.get(id)
            .map(|&license| license)
            .ok_or(ParseError::UnknownLicenseId(id))
    }
}

impl fmt::Display for SpdxLicense {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.id().fmt(f)
    }
}

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = SpdxLicense;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a known license")
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        SpdxLicense::parse(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for SpdxLicense {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LicenseVisitor)
    }
}

impl Serialize for SpdxLicense {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.id())
    }
}

impl SpdxLicense {
    /// The current number of SPDX licenses. See [`spdx::Map`](type.Map.html) as
    /// a use case.
    ///
    /// **SemVer Compatibility:** This number is allowed to change between
    /// otherwise API-compatible versions.
    // Public version defined here in order to be placed in docs alongside other
    // items.
    pub const COUNT: usize = Self::_COUNT;

    /// Returns an iterator over all licenses.
    ///
    /// ```
    /// use oceanpkg::drop::license::SpdxLicense;
    ///
    /// let licenses = SpdxLicense::all();
    /// assert_eq!(licenses.len(), SpdxLicense::COUNT);
    /// ```
    #[inline]
    pub fn all() -> impl DoubleEndedIterator<Item = Self> + ExactSizeIterator {
        (0..(Self::COUNT as u16)).map(|l| unsafe { std::mem::transmute(l) })
    }

    /// Attempts to parse `input` and returns a [`ParseError`] on error.
    #[inline]
    pub fn parse<'a, I>(input: I) -> Result<Self, ParseError<'a>>
        where I: TryInto<Self, Error = ParseError<'a>>
    {
        input.try_into()
    }

    /// Returns the string identifier of this license.
    #[inline]
    pub const fn id(self) -> &'static str {
        Self::ID[self as usize]
    }

    /// Returns the full name of this license.
    #[inline]
    pub const fn name(self) -> &'static str {
        Self::NAME[self as usize]
    }

    /// Considered libre/free by the [Free Software Foundation
    /// (FSF)](https://www.fsf.org).
    #[inline]
    pub const fn is_libre(self) -> bool {
        Self::LIBRE[self as usize]
    }

    /// The license is approved by the [Open Source Initiative
    /// (OSI)](https://opensource.org).
    #[inline]
    pub const fn is_osi_approved(self) -> bool {
        Self::OSI[self as usize]
    }

    /// Returns whether the license is associated with [Creative
    /// Commons](https://creativecommons.org).
    #[inline]
    pub const fn is_creative_commons(self) -> bool {
        // CORRECTNESS: The ordering of `CcBy1` and `CC01` is *very important*
        // for this function to emit correct results. They need to be declared
        // as the first and last Creative Common licenses in the enumeration,
        // respectively, in order for this check to work.
        const MIN: usize = SpdxLicense::CcBy1 as usize;
        const MAX: usize = SpdxLicense::CC01 as usize;
        let val = self as usize;
        (val >= MIN) & (val <= MAX)
    }
}

/// An error returned when attempting to parse a [`SpdxLicense`] or [`Expr`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    /// Empty string provided.
    Empty,
    /// An error returned when a license name is unknown.
    UnknownLicenseId(&'a str),
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Empty => {
                write!(f, "empty string provided")
            },
            ParseError::UnknownLicenseId(id) => {
                write!(f, "'{}' is not a known license ID", id)
            },
        }
    }
}
