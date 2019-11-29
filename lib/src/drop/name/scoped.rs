//! Name in the format `<owner>/<drop>`.

use std::{
    convert::TryInto,
    fmt,
};
use super::{
    Name,
    QueryName,
    ValidateError,
};

/// Name in the format `<owner>/<drop>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `QueryName`
pub struct ScopedName<'a> {
    /// The namespace of the drop.
    pub scope: &'a Name,
    /// The drop's given name.
    pub name: &'a Name,
}

impl<'a> AsRef<QueryName<'a>> for ScopedName<'a> {
    #[inline]
    fn as_ref(&self) -> &QueryName<'a> {
        self.as_query()
    }
}

impl fmt::Display for ScopedName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.scope, self.name)
    }
}

impl PartialEq<str> for ScopedName<'_> {
    fn eq(&self, s: &str) -> bool {
        let mut parts = s.split('/');
        match (parts.next(), parts.next(), parts.next()) {
            (Some(scope), Some(name), None) => {
                self.scope == scope && self.name == name
            },
            _ => false,
        }
    }
}

impl PartialEq<ScopedName<'_>> for str {
    #[inline]
    fn eq(&self, n: &ScopedName) -> bool {
        n == self
    }
}

impl<'a> ScopedName<'a> {
    /// Attempts to create a new instance by parsing `name`.
    #[inline]
    pub fn parse<N>(name: N) -> Result<Self, ParseError>
        where N: TryInto<Self, Error = ParseError>
    {
        name.try_into()
    }

    /// Creates a new instance by verifying `scope` and `name`.
    #[inline]
    pub fn new(scope: &'a [u8], name: &'a [u8]) -> Result<Self, ParseError> {
        match Name::new(scope) {
            Ok(scope) => match Name::new(name) {
                Ok(name) => Ok(Self {
                    scope,
                    name,
                }),
                Err(err) => Err(ParseError::Name(err)),
            },
            Err(err) => Err(ParseError::Scope(err)),
        }
    }

    /// Creates a new instance without attempting to verify `scope` or `name`.
    #[inline]
    pub unsafe fn new_unchecked(scope: &'a [u8], name: &'a [u8]) -> Self {
        Self {
            scope: Name::new_unchecked(scope),
            name: Name::new_unchecked(name),
        }
    }

    /// Creates a new instance in the `core` namespace.
    #[inline]
    pub const fn core(name: &'a Name) -> Self {
        Self { scope: Name::CORE, name }
    }

    /// Creates a new instance in the `ocean` namespace.
    #[inline]
    pub const fn ocean(name: &'a Name) -> Self {
        Self { scope: Name::OCEAN, name }
    }

    /// Returns `self` as a `QueryName`.
    #[inline]
    pub fn as_query(&self) -> &QueryName<'a> {
        // SAFETY: Checked above that the memory layout of both is the same
        unsafe { &*(self as *const Self as *const QueryName) }
    }
}

/// Error returned when parsing into a [`ScopedName`](struct.ScopedName.html).
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
