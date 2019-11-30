//! A drop name that may or may not be scoped.

use std::{
    convert::TryInto,
    fmt,
    slice,
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

impl<'a, N: Into<&'a Name>> From<[N; 2]> for QueryName<'a> {
    #[inline]
    fn from([scope, name]: [N; 2]) -> Self {
        Self::new(scope.into(), name)
    }
}

impl<'a, S, N> From<(S, N)> for QueryName<'a>
where
    S: Into<Option<&'a Name>>,
    N: Into<&'a Name>,
{
    #[inline]
    fn from((scope, name): (S, N)) -> Self {
        Self::new(scope, name)
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
    /// Creates a new instance from `scope` and `name`.
    #[inline]
    pub fn new<S, N>(scope: S, name: N) -> Self
    where
        S: Into<Option<&'a Name>>,
        N: Into<&'a Name>,
    {
        Self { scope: scope.into(), name: name.into() }
    }

    /// Attempts to create a new instance by parsing `query`.
    #[inline]
    pub fn parse<Q>(query: Q) -> Result<Self, scoped::ParseError>
        where Q: TryInto<Self, Error = scoped::ParseError>
    {
        query.try_into()
    }

    /// Copies the data referred to by `self` and takes ownership of it.
    #[inline]
    pub fn into_owned(self) -> OwnedQueryName {
        self.into()
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

    /// Converts `self` to a simple name if it has no scope.
    #[inline]
    pub fn to_name(&self) -> Option<&'a Name> {
        if self.scope.is_none() {
            Some(self.name)
        } else {
            None
        }
    }

    /// Converts `self` into a slice of `Name`s.
    #[inline]
    pub fn as_names(&self) -> &[&'a Name] {
        self.to_scoped()
            .map(|s| s.as_names())
            .unwrap_or(slice::from_ref(&self.name))
    }
}

/// A drop name that may or may not be scoped, with ownership over its names.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `ScopedName`
pub struct OwnedQueryName {
    /// The scope scope if the query is scoped to a specific owner.
    ///
    /// If this scope is `None`, then drops are checked against the main trusted
    /// set of packages. It is not yet decided as to what goes there.
    pub scope: Option<Box<Name>>,
    /// The name of the drop itself.
    pub name: Box<Name>,
}

assert_eq_size!(OwnedQueryName, QueryName);
assert_eq_align!(OwnedQueryName, QueryName);

impl From<QueryName<'_>> for OwnedQueryName {
    #[inline]
    fn from(n: QueryName) -> Self {
        Self {
            scope: n.scope.map(|s| s.into()),
            name:  n.name.into(),
        }
    }
}

impl OwnedQueryName {
    /// Returns `self` as a [`QueryName`] reference.
    ///
    /// [`QueryName`]: struct.QueryName.html
    #[inline]
    pub fn as_query_name<'s>(&'s self) -> &'s QueryName<'s> {
        unsafe { &*(self as *const Self as *const QueryName) }
    }

    /// Returns a [`QueryName`] for `self`.
    ///
    /// [`QueryName`]: struct.QueryName.html
    #[inline]
    pub fn to_query_name<'s>(&'s self) -> QueryName<'s> {
        *self.as_query_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_name() {
        let names = Name::RESERVED_SCOPES;

        for &name in names {
            assert_eq!(name, QueryName::from(name));
            for &scope in names {
                assert_ne!(name, QueryName::new(scope, name));
            }
        }
    }

    #[test]
    fn eq_str() {
        fn test(query: QueryName) {
            fn bad_queries(query: &str) -> Vec<String> {
                let query = query.to_string();
                let mut result = vec![
                    query.to_uppercase(),
                    format!("{}/", query),
                    format!("/{}", query),
                    format!("/{}/", query),
                ];
                if query.contains("/") {
                    result.push(query.replace("/", ""));
                }
                result
            }

            let query_string = query.to_string();
            assert_eq!(query, *query_string);

            for bad_query in bad_queries(&query_string) {
                assert_ne!(query, *bad_query);
            }
        }

        let names = Name::RESERVED_SCOPES;

        for &name in names {
            test(name.into());
            for &scope in names {
                test(QueryName::new(scope, name));
            }
        }
    }
}
