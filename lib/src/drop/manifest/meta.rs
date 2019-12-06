use std::collections::BTreeMap;
use crate::drop::{
    source::Git,
    version::Version,
};

/// The value for the `meta` key in the drop manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Meta {
    /// The drop's name.
    pub name: String,

    /// The pretty name displayed when viewing a drop.
    pub display_name: Option<String>,

    /// What is this drop?
    pub description: String,

    /// The licenses used.
    ///
    /// This can be a single license or multiple delimited by "AND" or "OR".
    pub license: Option<String>,

    /// Authors of the drop.
    pub authors: Option<Vec<String>>,

    /// A path to the package's "README" file.
    pub readme: Option<String>,

    /// A path to the package's change log file.
    pub changelog: Option<String>,

    /// This drop's corner of the internet.
    pub homepage: Option<String>,

    /// The URL where docs live.
    pub documentation: Option<String>,

    // Tables: all types that serialize into maps (or "tables" in TOML)
    // them must be placed last to succeed.

    /// The git repository where this drop can be fetched from.
    ///
    /// Repository info is taKen from here.
    pub git: Option<Git>,

    /// The drop version.
    pub version: Version,

    /// The versions that this version conflicts with.
    pub conflicts: Option<BTreeMap<String, String>>,
}
