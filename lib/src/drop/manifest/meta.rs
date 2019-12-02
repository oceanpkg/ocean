use std::collections::BTreeMap;
use crate::drop::{
    license::Expr,
    Name,
    source::Git,
    version::Version,
};

/// The value for the `meta` key in the drop manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Meta<'a> {
    /// The drop's name.
    pub name: &'a Name,

    /// What is this drop?
    pub description: &'a str,

    /// The license used.
    pub license: Option<Expr>,

    /// Authors of the drop.
    pub authors: Option<Vec<&'a str>>,

    /// A path to the package's "README" file.
    pub readme: Option<&'a str>,

    /// A path to the package's change log file.
    pub changelog: Option<&'a str>,

    /// This drop's corner of the internet.
    pub homepage: Option<&'a str>,

    /// The URL where docs live.
    pub documentation: Option<&'a str>,

    // Tables: all types that serialize into maps (or "tables" in TOML)
    // them must be placed last to succeed.

    /// The git repository where this drop can be fetched from.
    ///
    /// Repository info is taKen from here.
    pub git: Option<Git<'a>>,

    /// The drop version.
    pub version: Version<'a>,

    /// The versions that this version conflicts with.
    pub conflicts: Option<BTreeMap<&'a Name, &'a str>>,
}
