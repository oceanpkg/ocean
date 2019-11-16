//! Drop names.

use std::{
    convert::TryFrom,
    ffi::{CStr, OsStr},
    fmt,
    str,
};
use crate::ext::OsStrExt;

/// A drop name that may or may not be scoped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `ScopedName`
pub struct DropQuery<'a> {
    /// The scope scope if the query is scoped to a specific owner.
    ///
    /// If this scope is `None`, then drops are checked against the main trusted
    /// set of packages. It is not yet decided as to what goes there.
    pub scope: Option<&'a ValidName>,
    /// The name of the drop itself.
    pub name: &'a ValidName,
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
        let index = bytes.iter().enumerate().find(|(_, &b)| b == b'/');
        if let Some((index, _)) = index {
            let scope = &bytes[..index];
            let name  = &bytes[(index + 1)..];
            ScopedName::new(scope, name).map(|n| n.into())
        } else {
            // No '/' means the query is only a name.
            ValidName::new(bytes)
                .map(|name| Self { scope: None, name })
                .map_err(|err| ParseError::Name(err))
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

impl<'a> From<ScopedName<'a>> for DropQuery<'a> {
    #[inline]
    fn from(n: ScopedName<'a>) -> Self {
        Self { scope: Some(n.scope), name: n.name }
    }
}

impl fmt::Display for DropQuery<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(scoped) = self.to_scoped() {
            scoped.fmt(f)
        } else {
            self.name.fmt(f)
        }
    }
}

impl<'a> DropQuery<'a> {
    /// Attempts to create a new instance by parsing `query`.
    #[inline]
    pub fn parse<Q>(query: Q) -> Result<Self, ParseError>
        where Self: TryFrom<Q, Error = ParseError>
    {
        TryFrom::try_from(query)
    }

    /// Converts `self` to a scoped name if it is one.
    #[inline]
    pub fn to_scoped(&self) -> Option<&ScopedName<'a>> {
        if self.scope.is_none() {
            None
        } else {
            // SAFETY: Checked above that the memory layout of both is the same
            Some(unsafe { &*(self as *const Self as *const ScopedName) })
        }
    }
}

/// A name in the format `<owner>/<drop>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `DropQuery`
pub struct ScopedName<'a> {
    /// The namespace of the drop.
    pub scope: &'a ValidName,
    /// The drop's given name.
    pub name: &'a ValidName,
}

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

impl<'a> AsRef<DropQuery<'a>> for ScopedName<'a> {
    #[inline]
    fn as_ref(&self) -> &DropQuery<'a> {
        self.as_query()
    }
}

impl fmt::Display for ScopedName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.scope, self.name)
    }
}

impl<'a> ScopedName<'a> {
    /// Attempts to create a new instance by parsing `name`.
    #[inline]
    pub fn parse<N>(name: N) -> Result<Self, ParseError>
        where Self: TryFrom<N, Error = ParseError>
    {
        TryFrom::try_from(name)
    }

    /// Creates a new instance by verifying `scope` and `name`.
    #[inline]
    pub fn new(scope: &'a [u8], name: &'a [u8]) -> Result<Self, ParseError> {
        match ValidName::new(scope) {
            Ok(scope) => match ValidName::new(name) {
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
            scope: ValidName::new_unchecked(scope),
            name: ValidName::new_unchecked(name),
        }
    }

    /// Creates a new instance in the `core` namespace.
    #[inline]
    pub const fn core(name: &'a ValidName) -> Self {
        Self { scope: ValidName::CORE, name }
    }

    /// Creates a new instance in the `ocean` namespace.
    #[inline]
    pub const fn ocean(name: &'a ValidName) -> Self {
        Self { scope: ValidName::OCEAN, name }
    }

    /// Returns `self` as a `DropQuery`.
    #[inline]
    pub fn as_query(&self) -> &DropQuery<'a> {
        // SAFETY: Checked above that the memory layout of both is the same
        unsafe { &*(self as *const Self as *const DropQuery) }
    }
}

/// A name valid for a scope scope or drop name.
///
/// Valid names are lowercase, non-empty, ASCII alphanumeric, and can have
/// dashes (`-`) anywhere except for the beginning or end.
///
/// Regex: `^[^-][0-9a-z-]+[^-]$`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidName(str);

// Allows for creating a `&ValidName` in a `const` from a `&str`.
macro_rules! valid_name {
    ($name:expr) => {
        {
            union Convert<'a> {
                s: &'a str,
                n: &'a ValidName,
            }
            Convert { s: $name }.n
        }
    };
}

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

impl AsRef<str> for ValidName {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<[u8]> for ValidName {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Display for ValidName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ValidName {
    /// The string "core".
    pub const CORE: &'static Self = unsafe { valid_name!("core") };

    /// The string "ocean".
    pub const OCEAN: &'static Self = unsafe { valid_name!("ocean") };

    /// Namespaces reserved to only be used only by Ocean.
    pub const RESERVED_SCOPES: &'static [&'static Self] = &[
        Self::CORE,
        Self::OCEAN,
    ];

    /// Attempts to create a new instance by parsing `name`.
    #[inline]
    pub fn new<'a, N>(name: N) -> Result<&'a Self, ValidateError>
        where &'a Self: TryFrom<N, Error = ValidateError>
    {
        TryFrom::try_from(name)
    }

    /// Creates a new instance without parsing `name`.
    pub unsafe fn new_unchecked<'a, B>(name: &'a B) -> &'a Self
        where B: ?Sized + AsRef<[u8]>
    {
        &*(name.as_ref() as *const [u8] as *const Self)
    }

    /// Returns whether `name` is valid.
    #[inline]
    pub fn is_valid<N: AsRef<[u8]>>(name: N) -> bool {
        // Monomorphization
        fn imp(bytes: &[u8]) -> bool {
            match (bytes.first(), bytes.last()) {
                // Cannot be empty or begin/end with '-'
                (None, _) | (Some(b'-'), _) | (_, Some(b'-')) => return false,
                _ => {},
            }
            bytes.iter().all(|&b| match b {
                b'0'..=b'9' |
                b'a'..=b'z' |
                b'-' => true,
                _ => false,
            })
        }
        imp(name.as_ref())
    }
}

/// An error returned when a `ValidName` could not be created.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidateError(());

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
