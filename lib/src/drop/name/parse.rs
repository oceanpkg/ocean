use std::{
    convert::{TryFrom, TryInto},
    ffi::{CStr, OsStr},
    fmt,
    marker::PhantomData,
    str,
};
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};
use crate::ext::OsStrExt;
use super::{
    Name,
    query::{self, Query},
    scoped::{self, ScopedName},
    ValidateError,
};

impl<'a> TryFrom<&'a str> for &'a Name {
    type Error = ValidateError;

    #[inline]
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }
}

impl TryFrom<&str> for Box<Name> {
    type Error = ValidateError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        <&Name>::try_from(s).map(|name| name.into_boxed())
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

// TODO: Reintroduce `TryFrom<&[u8]>`.
impl<'a, N> TryFrom<&'a str> for ScopedName<N>
where
    &'a str: TryInto<N>,
{
    type Error = scoped::ParseError<<&'a str as TryInto<N>>::Error>;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {
        let mut scope_iter = name.splitn(2, '/');
        match (scope_iter.next(), scope_iter.next()) {
            (None, _) => unreachable!(),
            (_, None) => {
                Err(scoped::ParseError::MissingSeparator)
            },
            (Some(scope), Some(name)) => {
                ScopedName { scope, name }.try_cast()
            },
        }
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
                ScopedName::parse(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis)
    }
}

impl<N> Serialize for ScopedName<N>
    where ScopedName<N>: fmt::Display
{
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

//==============================================================================

// TODO: Reintroduce `TryFrom<&[u8]>`.
impl<'a, N, V> TryFrom<&'a str> for Query<N, V>
where
    &'a str: TryInto<N>,
    &'a str: TryInto<V>,
{
    type Error = query::ParseError<
        <&'a str as TryInto<N>>::Error,
        <&'a str as TryInto<V>>::Error,
    >;

    #[inline]
    fn try_from(query: &'a str) -> Result<Self, Self::Error> {
        Query::<&str>::parse_liberal(query).try_cast()
    }
}

assert_impl_all!(Query<Box<Name>, String>: Deserialize<'static>);

impl<'de, N, V, NE, VE> Deserialize<'de> for Query<N, V>
where
    for<'a> &'a str: TryInto<Query<N, V>, Error = query::ParseError<NE, VE>>,
    query::ParseError<NE, VE>: fmt::Display,
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct Vis<N, V>(PhantomData<(N, V)>);

        impl<'de, N, V, NE, VE> Visitor<'de> for Vis<N, V>
        where
            for<'a> &'a str: TryInto<Query<N, V>, Error = query::ParseError<NE, VE>>,
            query::ParseError<NE, VE>: fmt::Display,
        {
            type Value = Query<N, V>;

            #[inline]
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a valid drop name")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                TryInto::<Self::Value>::try_into(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Vis(PhantomData))
    }
}

impl<N, V> Serialize for Query<N, V>
    where Query<N, V>: fmt::Display
{
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}
