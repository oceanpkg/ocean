//! End-user configuration of the local Ocean installation.
//!
//! This is tied heavily to `InstallTarget`.

pub mod file;
#[doc(inline)]
pub use self::file::{CfgFile, CfgFileFmt};

/// Represents the configuration of a local installation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cfg {
    // TODO: ???
}
