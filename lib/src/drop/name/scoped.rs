//! A drop name in the format `<scope>/<name>`.

use std::{
    convert::TryInto,
    fmt,
};
use super::{
    Name,
    ValidateError,
};

/// Name in the format `<owner>/<drop>`.
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord, Hash)]
#[repr(C)] // For `as_names` to always have the same order
pub struct ScopedName<Name = String> {
    /// The namespace of the drop.
    pub scope: Name,
    /// The drop's given name.
    pub name: Name,
}

assert_eq_size!(ScopedName<Box<Name>>,  ScopedName<&Name>);
assert_eq_align!(ScopedName<Box<Name>>, ScopedName<&Name>);

impl<A, B: Into<A>> From<[B; 2]> for ScopedName<A> {
    #[inline]
    fn from([scope, name]: [B; 2]) -> Self {
        Self::new(scope, name)
    }
}

impl<N, A, B> From<(A, B)> for ScopedName<N>
where
    A: Into<N>,
    B: Into<N>,
{
    #[inline]
    fn from((scope, name): (A, B)) -> Self {
        Self::new(scope, name)
    }
}

impl<A, B> PartialEq<ScopedName<B>> for ScopedName<A>
    where A: PartialEq<B>
{
    #[inline]
    fn eq(&self, other: &ScopedName<B>) -> bool {
        self.scope == other.scope && self.name == other.name
    }
}

impl<A: AsRef<str>> PartialEq<str> for ScopedName<A> {
    fn eq(&self, s: &str) -> bool {
        let mut parts = s.splitn(2, '/');
        match (parts.next(), parts.next()) {
            (Some(scope), Some(name)) => {
                self.scope.as_ref() == scope && self.name.as_ref() == name
            },
            _ => false,
        }
    }
}

// Seems redundant but required to make `assert_eq!` prettier.
impl<A: AsRef<str>> PartialEq<&str> for ScopedName<A> {
    #[inline]
    fn eq(&self, s: &&str) -> bool {
        *self == **s
    }
}

impl<A: AsRef<str>> PartialEq<ScopedName<A>> for str {
    #[inline]
    fn eq(&self, n: &ScopedName<A>) -> bool {
        n == self
    }
}

// Seems redundant but required to make `assert_eq!` prettier.
impl<A: AsRef<str>> PartialEq<ScopedName<A>> for &str {
    #[inline]
    fn eq(&self, n: &ScopedName<A>) -> bool {
        n == self
    }
}

impl<N: fmt::Display> fmt::Display for ScopedName<N> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.scope, self.name)
    }
}

impl<N> ScopedName<N> {
    /// Creates a new instance from `scope` and `name`.
    #[inline]
    pub fn new<A, B>(scope: A, name: B) -> Self
    where
        A: Into<N>,
        B: Into<N>,
    {
        Self { scope: scope.into(), name: name.into() }
    }

    /// Attempts to create a new instance by parsing `name`.
    #[inline]
    pub fn parse<A, NE>(name: A) -> Result<Self, ParseError<NE>>
        where A: TryInto<Self, Error = ParseError<NE>>
    {
        name.try_into()
    }

    /// Converts `self` into a new `ScopedName` by performing an [`Into`]
    /// conversion over all fields.
    ///
    /// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
    #[inline]
    pub fn cast<A>(self) -> ScopedName<A>
        where N: Into<A>,
    {
        self.map(Into::into)
    }

    /// Converts `self` into a new `Query` by performing an [`Into`] conversion
    /// over all fields.
    ///
    /// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
    pub fn try_cast<A>(self) -> Result<ScopedName<A>, ParseError<N::Error>>
        where N: TryInto<A>
    {
        let scope = match self.scope.try_into() {
            Err(error) => return Err(ParseError::Scope(error)),
            Ok(scope) => scope,
        };
        let name = match self.name.try_into() {
            Err(error) => return Err(ParseError::Name(error)),
            Ok(name) => name,
        };
        Ok(ScopedName { scope, name })
    }

    /// Takes shared references to the fields of this name.
    #[inline]
    pub fn as_ref(&self) -> ScopedName<&N> {
        ScopedName {
            scope: &self.scope,
            name:  &self.name,
        }
    }

    /// Takes a shared reference to the fields of this name as type `A`.
    #[inline]
    pub fn to_ref<A>(&self) -> ScopedName<&A>
    where
        N: AsRef<A>,
        A: ?Sized,
    {
        self.as_ref().map(AsRef::as_ref)
    }

    /// Takes mutable references to the fields of this name.
    #[inline]
    pub fn as_mut(&mut self) -> ScopedName<&mut N> {
        ScopedName {
            scope: &mut self.scope,
            name:  &mut self.name,
        }
    }

    /// Takes a mutable references to the fields of this name as type `A`.
    #[inline]
    pub fn to_mut<A>(&mut self) -> ScopedName<&mut A>
    where
        N: AsMut<A>,
        A: ?Sized,
    {
        self.as_mut().map(AsMut::as_mut)
    }

    /// Creates a new `ScopedName` by mapping the function over the fields of
    /// `self`.
    #[inline]
    pub fn map<A, F>(self, mut f: F) -> ScopedName<A>
        where F: FnMut(N) -> A
    {
        ScopedName {
            scope: f(self.scope),
            name:  f(self.name),
        }
    }

    /// Converts `self` into an array of names.
    #[inline]
    pub fn as_names_array(&self) -> &[N; 2] {
        // SAFETY: This type consists of exactly two `N`s
        unsafe { &*(self as *const Self as *const [N; 2]) }
    }

    /// Converts `self` into a slice of names.
    #[inline]
    pub fn as_names_slice(&self) -> &[N] {
        self.as_names_array()
    }
}

impl<'n, N: ?Sized> ScopedName<&'n N> {
    /// Returns the result of calling [`ToOwned::to_owned`] on the fields of
    /// `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use oceanpkg::drop::name::ScopedName;
    ///
    /// let name:  ScopedName<&str>   = ScopedName::new("core", "wget");
    /// let owned: ScopedName<String> = name.to_owned();
    ///
    /// assert_eq!(name, owned);
    /// ```
    ///
    /// [`ToOwned::to_owned`]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned
    #[inline]
    pub fn to_owned(&self) -> ScopedName<N::Owned>
        where N: ToOwned
    {
        self.map(ToOwned::to_owned)
    }
}

impl<'n> ScopedName<&'n Name> {
    /// Creates a new instance in the `core` namespace.
    #[inline]
    pub const fn core(name: &'n Name) -> Self {
        Self { scope: Name::CORE, name }
    }

    /// Creates a new instance in the `ocean` namespace.
    #[inline]
    pub const fn ocean(name: &'n Name) -> Self {
        Self { scope: Name::OCEAN, name }
    }

    /// Creates a new instance by verifying `scope` and `name`.
    #[inline]
    pub fn from_pair<S, N>(
        scope: &'n S,
        name:  &'n N,
    ) -> Result<Self, ParseError<ValidateError>>
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
    pub unsafe fn from_pair_unchecked<S, N>(scope: &'n S, name: &'n N) -> Self
    where
        S: ?Sized + AsRef<[u8]>,
        N: ?Sized + AsRef<[u8]>,
    {
        Self {
            scope: Name::new_unchecked(scope),
            name:  Name::new_unchecked(name),
        }
    }
}

/// Error returned when parsing a [`ScopedName`](struct.ScopedName.html).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParseError<NameError> {
    /// Could not parse the scope (what comes before the separator).
    Scope(NameError),
    /// Could not parse the drop's name itself.
    Name(NameError),
    /// The separator character ('/') was not found in a scoped name.
    MissingSeparator,
}

impl<N: fmt::Display> fmt::Display for ParseError<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Scope(error) => {
                write!(f, "failed to parse scope: {}", error)
            },
            Self::Name(error) => {
                write!(f, "failed to parse name: {}", error)
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
        fn test(name: ScopedName<&Name>) {
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
