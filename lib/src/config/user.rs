//! User configuration data.

use std::{collections::HashMap, ffi::OsStr, time::Duration};

/// Represents the configuration specific to the user.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserConfig {
    /// Whether to send logs to ensure correct behavior.
    pub send_logs: bool,
    /// How often to send logs if `send_logs` is `true`. The default is 1 week.
    pub send_logs_rate: Duration,
    /// Aliases for CLI commands.
    #[serde(rename = "alias")]
    pub aliases: HashMap<String, Command>,
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
    pub fn new() -> Self {
        const DAY_SECS: u64 = 86400;
        const WEEK_SECS: u64 = DAY_SECS * 7;
        Self {
            send_logs: false,
            send_logs_rate: Duration::from_secs(WEEK_SECS),
            aliases: HashMap::new(),
        }
    }

    /// Finds the command for `alias` and returns its parts as `&str`s.
    pub fn parse_alias<'a>(&'a self, alias: &str) -> Option<Vec<&'a str>> {
        match self.aliases.get(alias) {
            Some(Command::Unparsed(command)) => {
                Some(command.split_whitespace().collect())
            }
            Some(Command::Parsed(command)) => {
                Some(command.iter().map(|s| s.as_str()).collect())
            }
            None => None,
        }
    }

    /// Finds the command for `alias` and returns its parts as `&OsStr`s.
    pub fn parse_alias_os<'a>(&'a self, alias: &str) -> Option<Vec<&'a OsStr>> {
        match self.aliases.get(alias) {
            Some(Command::Unparsed(command)) => {
                Some(command.split_whitespace().map(|s| s.as_ref()).collect())
            }
            Some(Command::Parsed(command)) => {
                Some(command.iter().map(|s| s.as_ref()).collect())
            }
            None => None,
        }
    }
}

/// A CLI command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Command {
    /// The raw command string.
    Unparsed(String),
    /// The command and its arguments as a list.
    Parsed(Vec<String>),
}
