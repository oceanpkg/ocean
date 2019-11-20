//! Dependency specification information.

use super::Git;

/// The value for an element in the `dependencies` key in the manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dep<'a> {
    /// A simple version requirement string, e.g. `^1.0.0`.
    Simple(&'a str),
    /// Detailed requirements beyond just a version requirement.
    Detailed {
        /// The version requirement string, e.g. `^1.0.0`.
        version: &'a str,

        /// What git repository can it be fetched from if requested via git.
        git: Option<Git<'a>>,
    }
}

impl<'a> Dep<'a> {
    /// Returns the version requirement string, e.g. `^1.0.0`.
    #[inline]
    pub fn version(&self) -> &'a str {
        match self {
            Dep::Simple(version) | Dep::Detailed { version, .. } => version
        }
    }
}
