use std::collections::BTreeMap;
use crate::drop::name::query::OwnedQueryName;
use super::Git;

/// A mapping from drop names to dependency specification information.
pub type Deps = BTreeMap<OwnedQueryName, DepInfo>;

/// The value associated with an element listed in the `dependencies` key in the
/// manifest.
///
/// This is defined as an `enum` to allow for flexibility in parsing. Either a
/// simple string will be parsed, in which case it's a version number
/// (`Version`), or a list of key/value pairs will be parsed (`Detailed`).
///
/// In the future, this should be defined as a `struct` to ease usage in Rust
/// while retaining flexibility in parsing.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DepInfo {
    /// A simple version requirement string, e.g. `^1.0.0`.
    Version(String),
    /// Detailed requirements beyond just a version requirement.
    Detailed {
        /// The version requirement string, e.g. `^1.0.0`.
        version: String,

        /// What git repository can it be fetched from if requested via git as
        /// an alternative source. Note that this may differ from the
        /// dependency's own `git` field in its drop manifest.
        git: Option<Git>,

        /// Whether the dependency is optional. The default is `false`.
        #[serde(default)]
        optional: bool,
    },
}

impl DepInfo {
    /// Returns the version requirement string, e.g. `^1.0.0`.
    #[inline]
    pub fn version(&self) -> &str {
        match self {
            Self::Version(version) |
            Self::Detailed { version, .. } => version
        }
    }
}
