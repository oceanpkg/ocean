//! User configuration data.

use std::time::Duration;

/// Represents the configuration specific to the user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserConfig {
    /// Whether to send logs to ensure correct behavior.
    pub send_logs: bool,
    /// How often to send logs if `send_logs` is `true`. The default is 1 week.
    pub send_logs_rate: Duration,
}

impl Default for UserConfig {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl UserConfig {
    /// Creates a new default configuration.
    #[inline]
    pub const fn new() -> Self {
        const DAY_SECS: u64 = 86400;
        const WEEK_SECS: u64 = DAY_SECS * 7;
        Self {
            send_logs: false,
            send_logs_rate: Duration::from_secs(WEEK_SECS),
        }
    }
}
