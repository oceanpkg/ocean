use std::ffi::OsStr;

/// Extended functionality for
/// [`OsStr`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html).
pub trait OsStrExt {
    /// Attempts to retrieve the underlying bytes of `self`.
    fn try_as_bytes(&self) -> Option<&[u8]>;
}

impl OsStrExt for OsStr {
    #[inline]
    fn try_as_bytes(&self) -> Option<&[u8]> {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            Some(self.as_bytes())
        }

        // To get the bytes on non-Unix platforms, `OsStr` needs to be converted
        // to a `str` first.
        #[cfg(not(unix))]
        {
            s.to_str().map(|s| s.as_bytes())
        }
    }
}
