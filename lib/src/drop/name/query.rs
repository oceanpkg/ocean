//! A drop lookup in the form `(<scope>/)?<drop>(@<version>)?`.

use std::{
    cmp::Ordering,
    convert::TryInto,
    fmt,
};

/// A drop lookup in the form `(<scope>/)?<name>(@<version>)?`.
///
/// By default, [`String`] is used for the generic `Name` and `Version` types.
///
/// When performing [zero-copy] parsing, it is recommended to use `Query<&str>`.
///
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
/// [zero-copy]: https://en.wikipedia.org/wiki/Zero-copy
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord, Hash)]
pub struct Query<Name = String, Version = Name> {
    /// The drop's owner.
    pub scope: Option<Name>,
    /// The drop's name.
    pub name: Name,
    /// The drop's version requirement.
    pub version: Option<Version>,
}

impl<A, B, X, Y> PartialEq<Query<X, Y>> for Query<A, B>
where
    A: PartialEq<X>,
    B: PartialEq<Y>,
{
    fn eq(&self, other: &Query<X, Y>) -> bool {
        // Needed because `Option<T>` only implements `PartialEq` over itself.
        fn eq_option<A, B>(a: &Option<A>, b: &Option<B>) -> bool
            where A: PartialEq<B>
        {
            match (a, b) {
                (Some(a), Some(b)) => a == b,
                (None, None) => true,
                _ => false,
            }
        }
        eq_option(&self.scope, &other.scope) &&
            self.name == other.name &&
            eq_option(&self.version, &other.version)
    }
}

impl<N: fmt::Display, V: fmt::Display> fmt::Display for Query<N, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(scope) = self.scope.as_ref() {
            write!(f, "{}/", scope)?;
        }

        write!(f, "{}", self.name)?;

        if let Some(version) = self.version.as_ref() {
            write!(f, "@{}", version)?;
        }

        Ok(())
    }
}

impl<'a> Query<&'a str> {
    // Monomorphized form of `parse_liberal` to slightly reduce the instruction
    // count of the binary.
    fn _parse_liberal(query: &'a str) -> Self {
        let mut scope_iter = query.splitn(2, '/');
        let (scope, rest) = match (scope_iter.next(), scope_iter.next()) {
            (None, _) => unreachable!(),
            (Some(rest), None) => (None, rest),
            (scope, Some(rest)) => (scope, rest),
        };

        let mut version_iter = rest.splitn(2, '@');
        let (name, version) = match (version_iter.next(), version_iter.next()) {
            (None, _) => unreachable!(),
            (Some(name), version) => (name, version),
        };

        Self { scope, name, version }
    }
}

impl<N, V> Query<N, V> {
    /// Creates a new `Query` instance with `scope`, `name`, and `version`.
    ///
    /// This serves as a convenience to not need to deal with explicitly
    /// wrapping types with `Some`.
    #[inline]
    pub fn new<A, B, C>(scope: A, name: B, version: C) -> Self
    where
        A: Into<N>,
        B: Into<N>,
        C: Into<V>,
    {
        Self {
            scope:   Some(scope.into()),
            name:    name.into(),
            version: Some(version.into()),
        }
    }

    /// Attempts to create a new instance by strictly parsing `name`.
    #[inline]
    pub fn parse<Q, NE, VE>(query: Q) -> Result<Self, ParseError<NE, VE>>
        where Q: TryInto<Self, Error = ParseError<NE, VE>>
    {
        query.try_into()
    }

    /// Creates a new instance by parsing `query` in a non-strict manner.
    ///
    /// # Examples
    ///
    /// ```
    /// use oceanpkg::drop::name::Query;
    ///
    /// let scope   = "core";
    /// let name    = "ruby";
    /// let version = "2.6";
    ///
    /// let query_string = format!("{}/{}@{}", scope, name, version);
    /// let query = Query::<&str>::parse_liberal(&query_string);
    ///
    /// assert_eq!(query.scope,   Some(scope));
    /// assert_eq!(query.name,    name);
    /// assert_eq!(query.version, Some(version));
    ///
    /// assert_eq!(query.to_string(), query_string);
    /// ```
    #[inline]
    pub fn parse_liberal<'a>(query: &'a str) -> Self
    where
        &'a str: Into<N>,
        &'a str: Into<V>,
    {
        Query::_parse_liberal(query).cast()
    }

    /// Converts `self` into a new `Query` by performing an [`Into`] conversion
    /// over all fields.
    ///
    /// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
    #[inline]
    pub fn cast<A, B>(self) -> Query<A, B>
    where
        N: Into<A>,
        V: Into<B>,
    {
        Query {
            scope:   self.scope.map(Into::into),
            name:    self.name.into(),
            version: self.version.map(Into::into),
        }
    }

    /// Converts `self` into a new `Query` by performing an [`Into`] conversion
    /// over all fields.
    ///
    /// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
    pub fn try_cast<A, B>(self) -> Result<Query<A, B>, ParseError<N::Error, V::Error>>
    where
        N: TryInto<A>,
        V: TryInto<B>,
    {
        let scope = match self.scope.map(TryInto::try_into) {
            Some(Err(error)) => return Err(ParseError::Scope(error)),
            Some(Ok(error)) => Some(error),
            None => None,
        };
        let name = match self.name.try_into() {
            Err(error) => return Err(ParseError::Name(error)),
            Ok(name) => name,
        };
        let version = match self.version.map(TryInto::try_into) {
            Some(Err(error)) => return Err(ParseError::Version(error)),
            Some(Ok(error)) => Some(error),
            None => None,
        };
        Ok(Query { scope, name, version })
    }

    /// Takes a shared reference to the fields of this query.
    ///
    /// See [`Query::as_mut`](#method.as_mut) for the mutable equivalent.
    #[inline]
    pub fn as_ref(&self) -> Query<&N, &V> {
        Query {
            scope:   self.scope.as_ref(),
            name:    &self.name,
            version: self.version.as_ref(),
        }
    }

    /// Takes a shared reference to the fields of this query as type `A`.
    ///
    /// See [`Query::to_mut`](#method.to_mut) for the mutable equivalent.
    ///
    /// # Examples
    ///
    /// ```
    /// use oceanpkg::drop::name::Query;
    ///
    /// let query: Query<String> = //
    /// # Query::new("", "", "");
    /// let by_ref: Query<&str> = query.to_ref();
    ///
    /// assert_eq!(query, by_ref);
    /// ```
    #[inline]
    pub fn to_ref<A>(&self) -> Query<&A>
    where
        N: AsRef<A>,
        V: AsRef<A>,
        A: ?Sized,
    {
        Query {
            scope:   self.scope.as_ref().map(AsRef::as_ref),
            name:    self.name.as_ref(),
            version: self.version.as_ref().map(AsRef::as_ref),
        }
    }

    /// Takes a mutable reference to the fields of this query.
    ///
    /// See [`Query::as_ref`](#method.as_ref) for the immutable equivalent.
    #[inline]
    pub fn as_mut(&mut self) -> Query<&mut N, &mut V> {
        Query {
            scope:   self.scope.as_mut(),
            name:    &mut self.name,
            version: self.version.as_mut(),
        }
    }

    /// Takes a mutable reference to the fields of this query as type `A`.
    ///
    /// See [`Query::to_ref`](#method.to_ref) for the immutable equivalent.
    pub fn to_mut<A>(&mut self) -> Query<&mut A>
    where
        N: AsMut<A>,
        V: AsMut<A>,
        A: ?Sized,
    {
        Query {
            scope:   self.scope.as_mut().map(AsMut::as_mut),
            name:    self.name.as_mut(),
            version: self.version.as_mut().map(AsMut::as_mut),
        }
    }

    /// Performs a partial version comparison between `self` and `other`.
    ///
    /// Returns `None` if:
    /// - One version is `Some` and the other is `Some`.
    /// - `<V as PartialOrd<B>>::partial_cmp` returns `None`.
    #[inline]
    pub fn cmp_version<A, B>(&self, other: &Query<A, B>) -> Option<Ordering>
        where V: PartialOrd<B>
    {
        match (&self.version, &other.version) {
            (Some(this), Some(other)) => this.partial_cmp(other),
            (None, None) => Some(Ordering::Equal),
            _ => None,
        }
    }
}

impl<'n, 'v, N: ?Sized, V: ?Sized> Query<&'n N, &'v V> {
    /// Returns the result of calling [`ToOwned::to_owned`] on the fields of
    /// `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use oceanpkg::drop::name::Query;
    ///
    /// let query: Query<&str> = Query::new("core", "wget", "*");
    /// let owned: Query<String> = query.to_owned();
    ///
    /// assert_eq!(query, owned);
    /// ```
    ///
    /// [`ToOwned::to_owned`]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned
    pub fn to_owned(&self) -> Query<N::Owned, V::Owned>
    where
        N: ToOwned,
        V: ToOwned,
    {
        Query {
            scope:   self.scope.map(ToOwned::to_owned),
            name:    self.name.to_owned(),
            version: self.version.map(ToOwned::to_owned),
        }
    }
}

/// An error returned when parsing a `Query`.
pub enum ParseError<NameError, VersionError> {
    /// An error occurred when parsing the `scope` field.
    Scope(NameError),
    /// An error occurred when parsing the `name` field.
    Name(NameError),
    /// An error occurred when parsing the `version` field.
    Version(VersionError),
}

impl<N: fmt::Display, V: fmt::Display> fmt::Display for ParseError<N, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse query ")?;
        match self {
            Self::Scope(error)   => write!(f, "scope: {}", error),
            Self::Name(error)    => write!(f, "name: {}", error),
            Self::Version(error) => write!(f, "version: {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_liberal() {
        let cases: &[(&str, (Option<&str>, &str, Option<&str>))] = &[
            ("ocean",           (None,          "ocean",  None)),
            ("ocean@1",         (None,          "ocean",  Some("1"))),
            ("ocean/ocean@1",   (Some("ocean"), "ocean",  Some("1"))),
            ("ocean//ocean@1",  (Some("ocean"), "/ocean", Some("1"))),
            ("ocean//ocean@@1", (Some("ocean"), "/ocean", Some("@1"))),
        ];
        for &(query_string, (scope, name, version)) in cases {
            let query = Query::<&str>::parse_liberal(query_string);

            assert_eq!(query, Query { scope, name, version });
            assert_eq!(query_string, query.to_string());
        }
    }
}
