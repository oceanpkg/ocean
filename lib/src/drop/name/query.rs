use std::{
    convert::TryInto,
    fmt,
};
use super::{
    Name,
    scoped::{self, ScopedName},
};

/// A drop name that may or may not be scoped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `ScopedName`
pub struct QueryName<'a> {
    /// The scope scope if the query is scoped to a specific owner.
    ///
    /// If this scope is `None`, then drops are checked against the main trusted
    /// set of packages. It is not yet decided as to what goes there.
    pub scope: Option<&'a Name>,
    /// The name of the drop itself.
    pub name: &'a Name,
}

impl<'a> From<&'a Name> for QueryName<'a> {
    #[inline]
    fn from(name: &'a Name) -> Self {
        Self { scope: None, name }
    }
}

impl<'a> From<ScopedName<'a>> for QueryName<'a> {
    #[inline]
    fn from(n: ScopedName<'a>) -> Self {
        Self { scope: Some(n.scope), name: n.name }
    }
}

impl fmt::Display for QueryName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(scoped) = self.to_scoped() {
            scoped.fmt(f)
        } else {
            self.name.fmt(f)
        }
    }
}

impl PartialEq<Name> for QueryName<'_> {
    #[inline]
    fn eq(&self, n: &Name) -> bool {
        self.scope.is_none() && self.name == n
    }
}

// Seems redundant but required to make `assert_eq!` prettier.
impl PartialEq<&Name> for QueryName<'_> {
    #[inline]
    fn eq(&self, n: &&Name) -> bool {
        *self == **n
    }
}

impl PartialEq<QueryName<'_>> for Name {
    #[inline]
    fn eq(&self, n: &QueryName) -> bool {
        n == self
    }
}

// Seems redundant but required to make `assert_eq!` prettier.
impl PartialEq<QueryName<'_>> for &Name {
    #[inline]
    fn eq(&self, n: &QueryName) -> bool {
        n == self
    }
}

impl PartialEq<str> for QueryName<'_> {
    fn eq(&self, s: &str) -> bool {
        let mut parts = s.split('/');
        match (parts.next(), parts.next(), parts.next(), self.scope) {
            (Some(scope), Some(name), None, Some(self_scope)) => {
                self_scope == scope && self.name == name
            },
            (Some(name), None, None, None) => {
                self.name == name
            },
            _ => false,
        }
    }
}

// Seems redundant but required to make `assert_eq!` prettier.
impl PartialEq<&str> for QueryName<'_> {
    #[inline]
    fn eq(&self, s: &&str) -> bool {
        *self == **s
    }
}

impl PartialEq<QueryName<'_>> for str {
    #[inline]
    fn eq(&self, n: &QueryName) -> bool {
        n == self
    }
}

impl PartialEq<QueryName<'_>> for &str {
    #[inline]
    fn eq(&self, n: &QueryName) -> bool {
        n == self
    }
}

impl<'a> QueryName<'a> {
    /// Attempts to create a new instance by parsing `query`.
    #[inline]
    pub fn parse<Q>(query: Q) -> Result<Self, scoped::ParseError>
        where Q: TryInto<Self, Error = scoped::ParseError>
    {
        query.try_into()
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
