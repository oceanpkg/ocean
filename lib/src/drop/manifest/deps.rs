use std::collections::BTreeMap;
use crate::drop::name::QueryName;
use super::Git;

/// A mapping from drop names to dependency specification information.
pub type Deps<'a> = BTreeMap<QueryName<'a>, DepInfo<'a>>;

flexible! {
    /// The value associated with an element listed in the `dependencies` key in the
    /// manifest.
    ///
    /// This is defined as an `enum` to allow for flexibility in parsing. Either a
    /// simple string will be parsed, in which case it's a version number
    /// (`Version`), or a list of key/value pairs will be parsed (`Detailed`).
    ///
    /// In the future, this should be defined as a `struct` to ease usage in Rust
    /// while retaining flexibility in parsing.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
    pub struct DepInfo<'a> {
        /// The version requirement string, e.g. `^1.0.0`.
        pub version: &'a str,

        /// Whether the dependency is optional. The default is `false`.
        #[serde(default)]
        pub optional: bool,

        // Tables: all types that serialize into maps (or "tables" in TOML)
        // them must be placed last to succeed.

        /// What git repository can it be fetched from if requested via git as
        /// an alternative source. Note that this may differ from the
        /// dependency's own `git` field in its drop manifest.
        pub git: Option<Git<'a>>,
    }
}

impl<'a> From<&'a str> for DepInfo<'a> {
    fn from(version: &'a str) -> Self {
        Self {
            version,
            git: None,
            optional: false,
        }
    }
}
