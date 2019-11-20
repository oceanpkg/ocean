use std::{
    fmt,
    convert::TryInto,
};

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

impl AsRef<str> for ValidName {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
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

    /// Converts `self` to the underlying UTF-8 string slice.
    #[inline]
    pub const fn as_str(&self) -> &str {
        &self.0
    }
}

/// An error returned when a `ValidName` could not be created.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidateError(pub(super) ());

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to validate drop name")
    }
}
