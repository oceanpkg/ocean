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
    Name,
    QueryName,
    QueryNameRef,
    scoped::{self, ScopedName, ScopedNameRef},
    ValidateError,
};

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

impl<'de> Deserialize<'de> for Box<Name> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = Box<Name>;

            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid drop name")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Name::new(v)
                    .map(Into::into)
                    .map_err(E::custom)
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                Name::new(v)
                    .map(Into::into)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a Name {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = &'de Name;

            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid drop name")
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Name::new(v).map_err(E::custom)
            }

            #[inline]
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                Name::new(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis)
    }
}

impl Serialize for Name {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

//==============================================================================

impl<'a> TryFrom<&'a str> for ScopedNameRef<'a> {
    type Error = scoped::ParseError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for ScopedNameRef<'a> {
    type Error = scoped::ParseError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let index = bytes.iter().enumerate().find(|(_, &b)| b == b'/');
        if let Some((index, _)) = index {
            let scope = &bytes[..index];
            let name  = &bytes[(index + 1)..];
            Self::from_pair(scope, name)
        } else {
            Err(scoped::ParseError::MissingSeparator)
        }
    }
}

impl<'a> TryFrom<&'a OsStr> for ScopedNameRef<'a> {
    type Error = scoped::ParseError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(scoped::ParseError::Name(ValidateError(())))
            .and_then(TryFrom::try_from)
    }
}

impl<'de> Deserialize<'de> for ScopedName {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = ScopedName;

            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid drop name")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                ScopedNameRef::parse(v)
                    .map(Into::into)
                    .map_err(E::custom)
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                ScopedNameRef::parse(v)
                    .map(Into::into)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for ScopedNameRef<'a> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = ScopedNameRef<'de>;

            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid scoped drop name")
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where E: de::Error
            {
                ScopedNameRef::parse(v).map_err(E::custom)
            }

            #[inline]
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                ScopedNameRef::parse(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis)
    }
}

impl Serialize for ScopedNameRef<'_> {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

//==============================================================================

impl<'a> TryFrom<&'a str> for QueryNameRef<'a> {
    type Error = scoped::ParseError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl<'a> TryFrom<&'a [u8]> for QueryNameRef<'a> {
    type Error = scoped::ParseError;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        match ScopedNameRef::parse(bytes) {
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

impl<'a> TryFrom<&'a OsStr> for QueryNameRef<'a> {
    type Error = scoped::ParseError;

    fn try_from(s: &'a OsStr) -> Result<Self, Self::Error> {
        s.try_as_bytes()
            .ok_or(scoped::ParseError::Name(ValidateError(())))
            .and_then(TryFrom::try_from)
    }
}

impl<'de> Deserialize<'de> for QueryName {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = QueryName;

            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid drop name")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                QueryNameRef::parse(v)
                    .map(Into::into)
                    .map_err(E::custom)
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                QueryNameRef::parse(v)
                    .map(Into::into)
                    .map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for QueryNameRef<'a> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = QueryNameRef<'de>;

            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid drop name")
            }

            #[inline]
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                where E: de::Error
            {
                QueryNameRef::parse(v).map_err(E::custom)
            }

            #[inline]
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                QueryNameRef::parse(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis)
    }
}

impl Serialize for QueryName {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

impl Serialize for QueryNameRef<'_> {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}
