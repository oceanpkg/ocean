//! Drop names.

use std::{
    convert::TryInto,
    fmt,
};

mod parse;
mod query;
pub mod scoped;

#[doc(inline)]
pub use self::{
    query::QueryName,
    scoped::ScopedName,
};

/// A valid drop name.
///
/// Valid names are non-empty, lowercase ASCII alphanumeric, and can have dashes
/// (`-`) anywhere except for the beginning or end.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(str);

// Allows for creating a `&Name` in a `const` from a `&str`.
macro_rules! valid_name {
    ($name:expr) => {
        {
            union Convert<'a> {
                s: &'a str,
                n: &'a Name,
            }
            Convert { s: $name }.n
        }
    };
}

impl AsRef<str> for Name {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for Name {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl PartialEq<str> for Name {
    #[inline]
    fn eq(&self, s: &str) -> bool {
        self.0 == *s
    }
}

impl PartialEq<[u8]> for Name {
    #[inline]
    fn eq(&self, b: &[u8]) -> bool {
        self.0.as_bytes() == b
    }
}

impl PartialEq<Name> for str {
    #[inline]
    fn eq(&self, n: &Name) -> bool {
        *self == n.0
    }
}

impl PartialEq<Name> for [u8] {
    #[inline]
    fn eq(&self, n: &Name) -> bool {
        self == n.0.as_bytes()
    }
}

impl fmt::Display for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Name {
    /// The string "core".
    pub const CORE: &'static Self = unsafe { valid_name!("core") };

    /// The string "ocean".
    pub const OCEAN: &'static Self = unsafe { valid_name!("ocean") };

    /// The string "self".
    pub const SELF: &'static Self = unsafe { valid_name!("self") };

    /// Namespaces reserved to only be used only by Ocean.
    pub const RESERVED_SCOPES: &'static [&'static Self] = &[
        Self::CORE,
        Self::OCEAN,
        Self::SELF,
    ];

    /// Attempts to create a new instance by parsing `name`.
    #[inline]
    pub fn new<'a, N>(name: N) -> Result<&'a Self, ValidateError>
        where N: TryInto<&'a Self, Error = ValidateError>
    {
        name.try_into()
    }

    /// Creates a new instance without parsing `name`.
    pub unsafe fn new_unchecked<'a, N>(name: &'a N) -> &'a Self
        where N: ?Sized + AsRef<[u8]>
    {
        &*(name.as_ref() as *const [u8] as *const Self)
    }

    /// Returns whether `name` is valid.
    #[inline]
    pub fn is_valid<N: AsRef<[u8]>>(name: N) -> bool {
        // Monomorphized
        fn imp(bytes: &[u8]) -> bool {
            match (bytes.first(), bytes.last()) {
                // Cannot be empty or begin/end with '-'
                (None, _) |
                (Some(b'-'), _) |
                (_, Some(b'-')) => false,
                _ => bytes.iter().all(|&b| match b {
                    b'0'..=b'9' |
                    b'a'..=b'z' |
                    b'-' => true,
                    _ => false,
                }),
            }
        }
        imp(name.as_ref())
    }

    /// Converts `self` to the underlying UTF-8 string slice.
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether Ocean reserves the right to use this name as a scope.
    #[inline]
    pub fn is_reserved_scope(&self) -> bool {
        Self::RESERVED_SCOPES.contains(&self)
    }
}

/// Error returned when a [`Name`](struct.Name.html) could not be created.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidateError(pub(super) ());

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to validate drop name")
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
                Name::is_valid(&name),
                "{:?} found to be invalid",
                name
            );

            for &c2 in &inner {
                for &c3 in &outer {
                    let name: String = [c1, c2, c3].iter().collect();
                    assert!(
                        Name::is_valid(&name),
                        "{:?} found to be invalid",
                        name
                    );
                }
            }
        }
    }

    #[test]
    fn invalid_names() {
        assert!(!Name::is_valid(""));
        assert!(!Name::is_valid("-"));
        assert!(!Name::is_valid("--"));
        assert!(!Name::is_valid("---"));

        for &ch in &outer() {
            let names: &[&[char]] = &[
                &[ch, '-'],
                &['-', ch],
                &['-', ch, '-'],
            ];
            for name in names {
                let name: String = name.iter().cloned().collect();
                assert!(
                    !Name::is_valid(&name),
                    "{:?} found to to be valid",
                    name
                );
            }
        }
    }
}
