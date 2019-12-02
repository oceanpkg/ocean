//! Name in the format `<owner>/<drop>`.

use std::{
    convert::TryInto,
    fmt,
};
use super::{
    Name,
    QueryNameRef,
    ValidateError,
};

/// Name in the format `<owner>/<drop>`, with ownership over its names.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `QueryNameRef`
pub struct ScopedName {
    /// The namespace of the drop.
    pub scope: Box<Name>,
    /// The drop's given name.
    pub name: Box<Name>,
}

assert_eq_size!(ScopedName,  ScopedNameRef);
assert_eq_align!(ScopedName, ScopedNameRef);

impl From<ScopedNameRef<'_>> for ScopedName {
    #[inline]
    fn from(n: ScopedNameRef) -> Self {
        Self {
            scope: n.scope.into(),
            name:  n.name.into(),
        }
    }
}

impl PartialEq<ScopedNameRef<'_>> for ScopedName {
    fn eq(&self, other: &ScopedNameRef) -> bool {
        &*self.scope == other.scope && &*self.name == other.name
    }
}

impl PartialEq<ScopedName> for ScopedNameRef<'_> {
    fn eq(&self, other: &ScopedName) -> bool {
        other == self
    }
}

impl fmt::Display for ScopedName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.as_ref() as &ScopedNameRef).fmt(f)
    }
}

impl ScopedName {
    /// Returns `self` as a [`ScopedNameRef`] reference.
    ///
    /// [`ScopedNameRef`]: struct.ScopedNameRef.html
    #[inline]
    pub fn as_ref<'s>(&'s self) -> &'s ScopedNameRef<'s> {
        unsafe { &*(self as *const Self as *const ScopedNameRef) }
    }

    /// Returns a [`ScopedNameRef`] for `self`.
    ///
    /// [`ScopedNameRef`]: struct.ScopedNameRef.html
    #[inline]
    pub fn to_ref<'s>(&'s self) -> ScopedNameRef<'s> {
        *self.as_ref()
    }
}

/// Name in the format `<owner>/<drop>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `QueryNameRef`
pub struct ScopedNameRef<'a> {
    /// The namespace of the drop.
    pub scope: &'a Name,
    /// The drop's given name.
    pub name: &'a Name,
}

impl<'a, N: Into<&'a Name>> From<[N; 2]> for ScopedNameRef<'a> {
    #[inline]
    fn from([scope, name]: [N; 2]) -> Self {
        Self::new(scope, name)
    }
}

impl<'a, S, N> From<(S, N)> for ScopedNameRef<'a>
where
    S: Into<&'a Name>,
    N: Into<&'a Name>,
{
    #[inline]
    fn from((scope, name): (S, N)) -> Self {
        Self::new(scope, name)
    }
}

impl<'a> AsRef<QueryNameRef<'a>> for ScopedNameRef<'a> {
    #[inline]
    fn as_ref(&self) -> &QueryNameRef<'a> {
        self.as_query()
    }
}

impl fmt::Display for ScopedNameRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.scope, self.name)
    }
}

impl PartialEq<str> for ScopedNameRef<'_> {
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
impl PartialEq<&str> for ScopedNameRef<'_> {
    #[inline]
    fn eq(&self, s: &&str) -> bool {
        *self == **s
    }
}

impl PartialEq<ScopedNameRef<'_>> for str {
    #[inline]
    fn eq(&self, n: &ScopedNameRef) -> bool {
        n == self
    }
}

impl PartialEq<ScopedNameRef<'_>> for &str {
    #[inline]
    fn eq(&self, n: &ScopedNameRef) -> bool {
        n == self
    }
}

impl<'a> ScopedNameRef<'a> {
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

    /// Copies the data referred to by `self` and takes ownership of it.
    #[inline]
    pub fn into_owned(self) -> ScopedName {
        self.into()
    }

    /// Returns `self` as a `QueryNameRef`.
    #[inline]
    pub fn as_query(&self) -> &QueryNameRef<'a> {
        // SAFETY: Checked above that the memory layout of both is the same
        unsafe { &*(self as *const Self as *const QueryNameRef) }
    }

    /// Converts `self` into a slice of `Name`s.
    #[inline]
    pub fn as_names(&self) -> &[&'a Name] {
        // SAFETY: This type consists of exactly two `Name` references.
        unsafe { &*(self as *const Self as *const [&Name; 2]) }
    }
}

/// Error returned when parsing into a [`ScopedNameRef`](struct.ScopedNameRef.html).
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
            Self::Scope(error) => {
                write!(f, "could not parse scope: {}", error)
            },
            Self::Name(error) => {
                write!(f, "could not parse name: {}", error)
            },
            Self::MissingSeparator => {
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
        fn test(name: ScopedNameRef) {
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
                test(ScopedNameRef { scope, name });
            }
        }
    }
}
