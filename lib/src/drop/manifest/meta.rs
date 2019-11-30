use std::collections::BTreeMap;
use crate::drop::{
    license::Expr,
    name::{
        Name,
        query::OwnedQueryName,
    },
};
use super::{Git, Version};

/// The value for the `meta` key in the drop manifest.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Meta {
    /// The drop's name.
    pub name: Box<Name>,

    /// What is this drop?
    pub description: String,

    /// The drop version.
    pub version: Version,

    /// The versions that this version conflicts with.
    pub conflicts: Option<BTreeMap<OwnedQueryName, String>>,

    /// The license used.
    pub license: Option<Expr>,

    /// Authors of the drop.
    pub authors: Option<Vec<String>>,

    /// A path to the package's "README" file.
    pub readme: Option<String>,

    /// A path to the package's change log file.
    pub changelog: Option<String>,

    /// The git repository where this drop can be fetched from.
    ///
    /// Repository info is taKen from here.
    pub git: Option<Git>,

    /// This drop's corner of the internet.
    pub homepage: Option<String>,

    /// The URL where docs live.
    pub documentation: Option<String>,
}
