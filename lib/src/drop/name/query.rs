//! A drop name that may or may not be scoped.

use std::{
    convert::TryInto,
    fmt,
    slice,
};
use super::{
    Name,
    scoped::{self, ScopedNameRef},
};

/// A drop name that may or may not be scoped, with ownership over its names.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `ScopedNameRef`
pub struct QueryName {
    /// The scope scope if the query is scoped to a specific owner.
    ///
    /// If this scope is `None`, then drops are checked against the main trusted
    /// set of packages. It is not yet decided as to what goes there.
    pub scope: Option<Box<Name>>,
    /// The name of the drop itself.
    pub name: Box<Name>,
}

assert_eq_size!(QueryName,  QueryNameRef);
assert_eq_align!(QueryName, QueryNameRef);

impl From<QueryNameRef<'_>> for QueryName {
    #[inline]
    fn from(n: QueryNameRef) -> Self {
        Self {
            scope: n.scope.map(|s| s.into()),
            name:  n.name.into(),
        }
    }
}

impl PartialEq<QueryNameRef<'_>> for QueryName {
    fn eq(&self, other: &QueryNameRef) -> bool {
        let eq_scope = match (&self.scope, other.scope) {
            (Some(self_scope), Some(other_scope)) => {
                &**self_scope == other_scope
            },
            (None, None) => true,
            _ => false,
        };
        eq_scope && &*self.name == other.name
    }
}

impl PartialEq<QueryName> for QueryNameRef<'_> {
    fn eq(&self, other: &QueryName) -> bool {
        other == self
    }
}

impl fmt::Display for QueryName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.as_ref() as &QueryNameRef).fmt(f)
    }
}

impl QueryName {
    /// Returns `self` as a [`QueryNameRef`] reference.
    ///
    /// [`QueryNameRef`]: struct.QueryNameRef.html
    #[inline]
    pub fn as_ref<'s>(&'s self) -> &'s QueryNameRef<'s> {
        unsafe { &*(self as *const Self as *const QueryNameRef) }
    }

    /// Returns a [`QueryNameRef`] for `self`.
    ///
    /// [`QueryNameRef`]: struct.QueryNameRef.html
    #[inline]
    pub fn to_ref<'s>(&'s self) -> QueryNameRef<'s> {
        *self.as_ref()
    }
}

/// A drop name that may or may not be scoped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // `scope` must be first to be bitwise-compatible with `ScopedNameRef`
pub struct QueryNameRef<'a> {
    /// The scope scope if the query is scoped to a specific owner.
    ///
    /// If this scope is `None`, then drops are checked against the main trusted
    /// set of packages. It is not yet decided as to what goes there.
    pub scope: Option<&'a Name>,
    /// The name of the drop itself.
    pub name: &'a Name,
}

impl<'a> From<&'a Name> for QueryNameRef<'a> {
    #[inline]
    fn from(name: &'a Name) -> Self {
        Self { scope: None, name }
    }
}

impl<'a, N: Into<&'a Name>> From<[N; 2]> for QueryNameRef<'a> {
    #[inline]
    fn from([scope, name]: [N; 2]) -> Self {
        Self::new(scope.into(), name)
    }
}

impl<'a, S, N> From<(S, N)> for QueryNameRef<'a>
where
    S: Into<Option<&'a Name>>,
    N: Into<&'a Name>,
{
    #[inline]
    fn from((scope, name): (S, N)) -> Self {
        Self::new(scope, name)
    }
}

impl<'a> From<ScopedNameRef<'a>> for QueryNameRef<'a> {
    #[inline]
    fn from(n: ScopedNameRef<'a>) -> Self {
        Self { scope: Some(n.scope), name: n.name }
    }
}

impl fmt::Display for QueryNameRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(scoped) = self.to_scoped() {
            scoped.fmt(f)
        } else {
            self.name.fmt(f)
        }
    }
}

impl PartialEq<Name> for QueryNameRef<'_> {
    #[inline]
    fn eq(&self, n: &Name) -> bool {
        self.scope.is_none() && self.name == n
    }
}

// Seems redundant but required to make `assert_eq!` prettier.
impl PartialEq<&Name> for QueryNameRef<'_> {
    #[inline]
    fn eq(&self, n: &&Name) -> bool {
        *self == **n
    }
}

impl PartialEq<QueryNameRef<'_>> for Name {
    #[inline]
    fn eq(&self, n: &QueryNameRef) -> bool {
        n == self
    }
}

// Seems redundant but required to make `assert_eq!` prettier.
impl PartialEq<QueryNameRef<'_>> for &Name {
    #[inline]
    fn eq(&self, n: &QueryNameRef) -> bool {
        n == self
    }
}

impl PartialEq<str> for QueryNameRef<'_> {
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
impl PartialEq<&str> for QueryNameRef<'_> {
    #[inline]
    fn eq(&self, s: &&str) -> bool {
        *self == **s
    }
}

impl PartialEq<QueryNameRef<'_>> for str {
    #[inline]
    fn eq(&self, n: &QueryNameRef) -> bool {
        n == self
    }
}

impl PartialEq<QueryNameRef<'_>> for &str {
    #[inline]
    fn eq(&self, n: &QueryNameRef) -> bool {
        n == self
    }
}

impl<'a> QueryNameRef<'a> {
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
    pub fn into_owned(self) -> QueryName {
        self.into()
    }

    /// Converts `self` to a scoped name if it is one.
    #[inline]
    pub fn to_scoped(&self) -> Option<&ScopedNameRef<'a>> {
        if self.scope.is_none() {
            None
        } else {
            // SAFETY: Checked above that the memory layout of both is the same
            Some(unsafe { &*(self as *const Self as *const ScopedNameRef) })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_name() {
        let names = Name::RESERVED_SCOPES;

        for &name in names {
            assert_eq!(name, QueryNameRef::from(name));
            for &scope in names {
                assert_ne!(name, QueryNameRef::new(scope, name));
            }
        }
    }

    #[test]
    fn eq_str() {
        fn test(query: QueryNameRef) {
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
                test(QueryNameRef::new(scope, name));
            }
        }
    }
}
