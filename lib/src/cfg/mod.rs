//! End-user configuration of the local Ocean installation.
//!
//! This is tied heavily to `InstallTarget`.

use std::time::Duration;

pub mod file;
#[doc(inline)]
pub use self::file::{CfgFile, CfgFileFmt};

/// Represents the configuration of a local installation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cfg {
    /// Whether to send logs to ensure correct behavior.
    pub send_logs: bool,
    /// How often to send logs if `send_logs` is `true`. The default is 1 week.
    pub send_logs_rate: Duration,
}

impl Default for Cfg {
    fn default() -> Self {
        const DAY_SECS: u64 = 86400;
        const WEEK_SECS: u64 = DAY_SECS * 7;
        Cfg {
            send_logs: false,
            send_logs_rate: Duration::from_secs(WEEK_SECS),
        }
    }
}
