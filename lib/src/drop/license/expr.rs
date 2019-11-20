//! License expressions.

use std::{
    convert::{TryFrom, TryInto},
    fmt,
};
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};
use super::{License, SpdxLicense};

/// A license expression.
///
/// Grammar (note the padded spacing):
///
/// ```txt
/// Single = License
///
/// Or = License " OR " License
///    | License " OR " Or
///
/// And = License " AND " License
///     | License " AND " And
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expr<'a> {
    /// Only one license to follow.
    Single(License<'a>),
    /// Two or more licenses
    Or(Or<'a>),
    ///
    And(And<'a>),
}

impl From<SpdxLicense> for Expr<'_> {
    #[inline]
    fn from(l: SpdxLicense) -> Self {
        Self::Single(l.into())
    }
}

impl<'a> From<License<'a>> for Expr<'a> {
    #[inline]
    fn from(l: License<'a>) -> Self {
        Self::Single(l)
    }
}

impl<'a> From<Or<'a>> for Expr<'a> {
    #[inline]
    fn from(or: Or<'a>) -> Self {
        Self::Or(or)
    }
}

impl<'a> From<And<'a>> for Expr<'a> {
    #[inline]
    fn from(and: And<'a>) -> Self {
        Self::And(and)
    }
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Single(ref l) => l.fmt(f),
            Expr::Or(ref or) => or.fmt(f),
            Expr::And(ref and) => and.fmt(f),
        }
    }
}

// TODO: Implement `TryFrom<&[u8]>` for `Expr`
impl<'a> TryFrom<&'a str> for Expr<'a> {
    type Error = ParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use std::{iter, str::Split};

        fn handle_iter<'a>(
            a: &'a str,
            b: &'a str,
            iter: Split<'a, &str>,
        ) -> Vec<License<'a>> {
            iter::once(a)
                .chain(iter::once(b))
                .chain(iter)
                .map(str::trim)
                .map(License::from)
                .collect()
        }

        let s = s.trim();
        let mut iter_or = s.split(" OR ");

        match (iter_or.next(), iter_or.next()) {
            (None, _) => {
                Err(ParseError::Empty)
            },
            (Some(s), None) => {
                let mut iter_and = s.split(" AND ");

                match (iter_and.next(), iter_and.next()) {
                    (None, _) => {
                        Err(ParseError::Empty)
                    },
                    (Some(s), None) => {
                        Ok(Self::Single(s.into()))
                    },
                    (Some(l1), Some(l2)) => {
                        Ok(And(handle_iter(l1, l2, iter_and)).into())
                    },
                }
            },
            (Some(l1), Some(l2)) => {
                Ok(Or(handle_iter(l1, l2, iter_or)).into())
            },
        }
    }
}

struct ExprVisitor;

impl<'de> Visitor<'de> for ExprVisitor {
    type Value = Expr<'de>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "license expression")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Expr::parse(v).map_err(E::custom)
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Expr<'a> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(ExprVisitor)
    }
}

impl Serialize for Expr<'_> {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

impl<'a> Expr<'a> {
    /// Attempts to parse `input` and returns a
    /// [`ParseError`](struct.ParseError.html) on error.
    #[inline]
    pub fn parse<I>(input: I) -> Result<Self, ParseError>
        where I: TryInto<Self, Error = ParseError>
    {
        input.try_into()
    }

    /// Returns the underlying slice of licenses.
    #[inline]
    pub fn as_slice(&self) -> &[License<'a>] {
        match self {
            Expr::Single(l) => std::slice::from_ref(l),
            Expr::Or(or) => or.as_slice(),
            Expr::And(and) => and.as_slice(),
        }
    }
}

/// The error returned when parsing a [`Expr`](enum.Expr.html) fails.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// The expression was found to be empty.
    Empty,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "missing text in license string")
        }
    }
}

/// A set of licenses separated by `OR`. For projects that are dual/n-ary
/// licensed.
///
/// Instances of this type always have two or more.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Or<'a>(Vec<License<'a>>);

impl<'a> Or<'a> {
    /// Returns the underlying slice of licenses.
    #[inline]
    pub fn as_slice(&self) -> &[License<'a>] {
        self.0.as_slice()
    }
}

/// A set of licenses separated by `AND`. For projects that require restrictions
/// of multiple licenses.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct And<'a>(Vec<License<'a>>);

impl<'a> And<'a> {
    /// Returns the underlying slice of licenses.
    #[inline]
    pub fn as_slice(&self) -> &[License<'a>] {
        self.0.as_slice()
    }
}

mod impl_display {
    use super::*;

    type Iter<'a> = std::slice::Iter<'a, License<'a>>;

    fn display_iter(mut iter: Iter, sep: &str, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(first) = iter.next() {
            fmt::Display::fmt(&first, f)?;
        } else {
            return Ok(());
        }
        for next in iter {
            write!(f, "{}{}", sep, next)?;
        }
        Ok(())
    }

    impl fmt::Display for Or<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            display_iter(self.0.iter(), " OR ", f)
        }
    }

    impl fmt::Display for And<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            display_iter(self.0.iter(), " AND ", f)
        }
    }
}
