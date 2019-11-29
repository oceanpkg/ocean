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
    QueryName,
    scoped::{self, ScopedName},
    ValidateError,
    Name,
};

impl<'a> TryFrom<&'a str> for QueryName<'a> {
    type Error = scoped::ParseError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for QueryName<'a> {
    type Error = scoped::ParseError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        match ScopedName::parse(bytes) {
            Ok(scoped) => Ok(scoped.into()),
            Err(scoped::ParseError::MissingSeparator) => {
                // No '/' means the query is only a name.
                Name::new(bytes)
                    .map(|name| Self { scope: None, name })
                    .map_err(|err| scoped::ParseError::Name(err))
            },
            Err(error) => Err(error),
        }
    }
}

impl<'a> TryFrom<&'a OsStr> for QueryName<'a> {
    type Error = scoped::ParseError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(scoped::ParseError::Name(ValidateError(())))
            .and_then(TryFrom::try_from)
    }
}

//==============================================================================

impl<'a> TryFrom<&'a str> for ScopedName<'a> {
    type Error = scoped::ParseError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for ScopedName<'a> {
    type Error = scoped::ParseError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let index = bytes.iter().enumerate().find(|(_, &b)| b == b'/');
        if let Some((index, _)) = index {
            let scope = &bytes[..index];
            let name  = &bytes[(index + 1)..];
            Self::new(scope, name)
        } else {
            Err(scoped::ParseError::MissingSeparator)
        }
    }
}

impl<'a> TryFrom<&'a OsStr> for ScopedName<'a> {
    type Error = scoped::ParseError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(scoped::ParseError::Name(ValidateError(())))
            .and_then(TryFrom::try_from)
    }
}

//==============================================================================

impl<'a> TryFrom<&'a str> for &'a Name {
    type Error = ValidateError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for &'a Name {
    type Error = ValidateError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if Name::is_valid(bytes) {
            Ok(unsafe { &*(bytes as *const [u8] as *const Name) })
        } else {
            Err(ValidateError(()))
        }
    }
}

impl<'a> TryFrom<&'a CStr> for &'a Name {
    type Error = ValidateError;

    #[inline]
    fn try_from(s: &'a CStr) -> Result<Self, Self::Error> {
        Self::try_from(s.to_bytes())
    }
}

impl<'a> TryFrom<&'a OsStr> for &'a Name {
    type Error = ValidateError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(ValidateError(()))
            .and_then(TryFrom::try_from)
    }
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = &'de Name;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a valid drop name")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Name::new(v).map_err(E::custom)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a Name {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(NameVisitor)
    }
}

impl Serialize for Name {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
