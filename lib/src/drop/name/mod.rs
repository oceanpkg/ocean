//! Drop names.

use std::{
    convert::TryFrom,
    fmt,
};

mod parse;
mod valid;

pub use self::{parse::*, valid::*};

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

#[cfg(test)]
mod tests {
    use super::*;

    // Legal outer characters
    fn outer() -> Vec<char> {
        let mut outer = Vec::new();
        outer.extend((b'a'..=b'z').map(|byte| byte as char));
        outer.extend((b'0'..=b'9').map(|byte| byte as char));
        outer
    }

    #[test]
    fn valid_names() {
        let outer = outer();
        let mut inner = outer.clone();
        inner.push('-');

        for &c1 in &outer {
            let mut name_buf = [0; 4];
            let name = c1.encode_utf8(&mut name_buf);
            assert!(
                ValidName::is_valid(&name),
                "{:?} found to be invalid",
                name
            );

            for &c2 in &inner {
                for &c3 in &outer {
                    let name: String = [c1, c2, c3].iter().collect();
                    assert!(
                        ValidName::is_valid(&name),
                        "{:?} found to be invalid",
                        name
                    );
                }
            }
        }
    }

    #[test]
    fn invalid_names() {
        assert!(!ValidName::is_valid(""));
        assert!(!ValidName::is_valid("-"));
        assert!(!ValidName::is_valid("--"));
        assert!(!ValidName::is_valid("---"));

        for &ch in &outer() {
            let names: &[&[char]] = &[
                &[ch, '-'],
                &['-', ch],
                &['-', ch, '-'],
            ];
            for name in names {
                let name: String = name.iter().cloned().collect();
                assert!(
                    !ValidName::is_valid(&name),
                    "{:?} found to to be valid",
                    name
                );
            }
        }
    }
}
