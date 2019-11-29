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

impl<'a, N: Into<&'a Name>> From<[N; 2]> for ScopedName<'a> {
    #[inline]
    fn from([scope, name]: [N; 2]) -> Self {
        Self::new(scope, name)
    }
}

impl<'a, S, N> From<(S, N)> for ScopedName<'a>
where
    S: Into<&'a Name>,
    N: Into<&'a Name>,
{
    #[inline]
    fn from((scope, name): (S, N)) -> Self {
        Self::new(scope, name)
    }
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

// Seems redundant but required to make `assert_eq!` prettier.
impl PartialEq<&str> for ScopedName<'_> {
    #[inline]
    fn eq(&self, s: &&str) -> bool {
        *self == **s
    }
}

impl PartialEq<ScopedName<'_>> for str {
    #[inline]
    fn eq(&self, n: &ScopedName) -> bool {
        n == self
    }
}

impl PartialEq<ScopedName<'_>> for &str {
    #[inline]
    fn eq(&self, n: &ScopedName) -> bool {
        n == self
    }
}

impl<'a> ScopedName<'a> {
    /// Creates a new instance from `scope` and `name`.
    #[inline]
    pub fn new<S, N>(scope: S, name: N) -> Self
    where
        S: Into<&'a Name>,
        N: Into<&'a Name>,
    {
        Self { scope: scope.into(), name: name.into() }
    }

    /// Attempts to create a new instance by parsing `name`.
    #[inline]
    pub fn parse<N>(name: N) -> Result<Self, ParseError>
        where N: TryInto<Self, Error = ParseError>
    {
        name.try_into()
    }

    /// Creates a new instance by verifying `scope` and `name`.
    #[inline]
    pub fn from_pair<S, N>(scope: &'a S, name: &'a N) -> Result<Self, ParseError>
    where
        S: ?Sized + AsRef<[u8]>,
        N: ?Sized + AsRef<[u8]>,
    {
        match Name::new(scope.as_ref()) {
            Ok(scope) => match Name::new(name.as_ref()) {
                Ok(name) => Ok(Self { scope, name }),
                Err(err) => Err(ParseError::Name(err)),
            },
            Err(err) => Err(ParseError::Scope(err)),
        }
    }

    /// Creates a new instance without attempting to verify `scope` or `name`.
    #[inline]
    pub unsafe fn from_pair_unchecked<S, N>(scope: &'a S, name: &'a N) -> Self
    where
        S: ?Sized + AsRef<[u8]>,
        N: ?Sized + AsRef<[u8]>,
    {
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

    /// Converts `self` into a slice of `Name`s.
    #[inline]
    pub fn as_names(&self) -> &[&'a Name] {
        // SAFETY: This type consists of exactly two `Name` references.
        unsafe { &*(self as *const Self as *const [&Name; 2]) }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_str() {
        fn test(name: ScopedName) {
            fn bad_names(name: &str) -> Vec<String> {
                let name = name.to_string();
                vec![
                    name.to_uppercase(),
                    format!("{}/", name),
                    format!("/{}", name),
                    format!("/{}/", name),
                    name.replace("/", ""),
                ]
            }

            let name_string = name.to_string();
            assert_eq!(name, *name_string);

            for bad_name in bad_names(&name_string) {
                assert_ne!(name, *bad_name);
            }
        }

        let names = Name::RESERVED_SCOPES;

        for &name in names {
            for &scope in names {
                test(ScopedName { scope, name });
            }
        }
    }
}
