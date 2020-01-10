//! Describes how drops are installed.
//!
//! Drops can either be installed specifically for the current user executing
//! Ocean or globally, making them available to _all_ users.

use std::{error::Error, fmt};

mod target;

#[doc(inline)]
pub use self::target::InstallTarget;

/// A directory for an `InstallTarget` could not be retrieved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirError {
    /// Could not get the current user's home directory.
    CurrentUserHome,
    /// Could not get the current user's configuration directory.
    CurrentUserConfigDir,
}

impl Error for DirError {}

impl fmt::Display for DirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CurrentUserHome => {
                write!(f, "Could not get current user's home directory")
            }
            Self::CurrentUserConfigDir => {
                write!(f, "Could not get current user's config directory")
            }
        }
    }
}
