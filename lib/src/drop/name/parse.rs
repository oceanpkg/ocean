use std::{
    convert::TryFrom,
    ffi::{CStr, OsStr},
    fmt,
    str,
};
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};
use crate::ext::OsStrExt;
use super::{
    DropQuery,
    ScopedName,
    ValidateError,
    ValidName,
};

/// An error returned when parsing into a `DropQuery` or `ScopedName`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseError {
    /// Could not parse the scope (what comes before the separator).
    Scope(ValidateError),
    /// Could not parse the drop's name itself.
    Name(ValidateError),
    /// The separator character ('/') was not found in a scoped name.
    MissingSeparator,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Scope(error) => {
                write!(f, "could not parse scope: {}", error)
            },
            ParseError::Name(error) => {
                write!(f, "could not parse name: {}", error)
            },
            ParseError::MissingSeparator => {
                write!(f, "missing '/' separator in scoped name")
            },
        }
    }
}

impl<'a> TryFrom<&'a str> for DropQuery<'a> {
    type Error = ParseError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for DropQuery<'a> {
    type Error = ParseError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        match ScopedName::parse(bytes) {
            Ok(scoped) => Ok(scoped.into()),
            Err(ParseError::MissingSeparator) => {
                // No '/' means the query is only a name.
                ValidName::new(bytes)
                    .map(|name| Self { scope: None, name })
                    .map_err(|err| ParseError::Name(err))
            },
            Err(error) => Err(error),
        }
    }
}

impl<'a> TryFrom<&'a OsStr> for DropQuery<'a> {
    type Error = ParseError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(ParseError::Name(ValidateError(())))
            .and_then(TryFrom::try_from)
    }
}

//==============================================================================

impl<'a> TryFrom<&'a str> for ScopedName<'a> {
    type Error = ParseError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for ScopedName<'a> {
    type Error = ParseError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let index = bytes.iter().enumerate().find(|(_, &b)| b == b'/');
        if let Some((index, _)) = index {
            let scope = &bytes[..index];
            let name  = &bytes[(index + 1)..];
            Self::new(scope, name)
        } else {
            Err(ParseError::MissingSeparator)
        }
    }
}

impl<'a> TryFrom<&'a OsStr> for ScopedName<'a> {
    type Error = ParseError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(ParseError::Name(ValidateError(())))
            .and_then(TryFrom::try_from)
    }
}

//==============================================================================

impl<'a> TryFrom<&'a str> for &'a ValidName {
    type Error = ValidateError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for &'a ValidName {
    type Error = ValidateError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if ValidName::is_valid(bytes) {
            Ok(unsafe { &*(bytes as *const [u8] as *const ValidName) })
        } else {
            Err(ValidateError(()))
        }
    }
}

impl<'a> TryFrom<&'a CStr> for &'a ValidName {
    type Error = ValidateError;

    #[inline]
    fn try_from(s: &'a CStr) -> Result<Self, Self::Error> {
        Self::try_from(s.to_bytes())
    }
}

impl<'a> TryFrom<&'a OsStr> for &'a ValidName {
    type Error = ValidateError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(ValidateError(()))
            .and_then(TryFrom::try_from)
    }
}

struct ValidNameVisitor;

impl<'de> Visitor<'de> for ValidNameVisitor {
    type Value = &'de ValidName;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a valid drop name")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ValidName::new(v).map_err(E::custom)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a ValidName {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(ValidNameVisitor)
    }
}

impl Serialize for ValidName {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
