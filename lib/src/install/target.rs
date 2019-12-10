use std::path::PathBuf;
use shared::ext::PathBufExt;
use super::DirError;

/// Indicates where to (un)install a drop.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InstallTarget {
    /// Installation available for the current user; the default.
    ///
    /// In most places, it is usually assumed that this is what's wanted.
    CurrentUser,
    /// Installation available for a user, specified by a name.
    ///
    /// On the command-line, this is specified by the `--user`/`-u` flag.
    SpecificUser(String),
    /// Globally-available installation.
    ///
    /// On the command-line, this is specified by the `--global`/`-g` flag.
    Global,
}

impl Default for InstallTarget {
    #[inline]
    fn default() -> Self {
        InstallTarget::CurrentUser
    }
}

impl InstallTarget {
    /// Returns the base directory for the installation target.
    pub fn base_dir(&self) -> Result<PathBuf, DirError> {
        #[cfg(unix)]
        match self {
            Self::CurrentUser => {
                // Usually means:
                // - $HOME/ocean
                // - ~/ocean
                // or anything of that nature; see `dirs` docs for more info.
                dirs::home_dir()
                    .ok_or(DirError::CurrentUserHome)
                    .map(|home| home.pushing("ocean"))
            },
            Self::SpecificUser(username) => {
                unimplemented!("TODO: Get base directory for {:?}", username);
            },
            Self::Global => {
                // TODO+SUDO: Needs admin access to write to either. Should be
                // in a separate process that runs based on user password input.
                // Essentially the same UX as when shells try to run something
                // prefixed with `sudo`
                if cfg!(target_os = "macos") {
                    Ok("/Library/Ocean".into())
                } else {
                    Ok("/usr/local/Ocean".into())
                }
            },
        }

        #[cfg(windows)]
        unimplemented!("TODO: Write & test on Windows :)");
    }

    /// Returns the configuration files directory for the installation target.
    ///
    /// # Examples
    ///
    /// For `InstallTarget::CurrentUser`, some expected outputs are:
    ///
    /// - Windows: `C:\Users\Alice\AppData\Roaming\Ocean`
    /// - macOS:   `/Users/Alice/Library/Preferences/Ocean`
    /// - Linux:   `/home/alice/.config`
    pub fn cfg_dir(&self) -> Result<PathBuf, DirError> {
        match self {
            Self::CurrentUser => {
                dirs::config_dir()
                    .ok_or(DirError::CurrentUserCfgDir)
                    .map(|cfg| cfg.pushing("Ocean"))
            },
            Self::SpecificUser(username) => {
                unimplemented!("TODO: Get config directory for {:?}", username);
            },
            Self::Global => {
                unimplemented!("TODO: Get global config directory");
            },
        }
    }
}
