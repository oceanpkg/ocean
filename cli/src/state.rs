use std::{
    borrow::Cow,
    env,
    path::{Path, PathBuf},
    time::Instant,
};
use failure::ResultExt;
use oceanpkg::{
    drop::name::Query,
    install::InstallTarget,
};

/// Resources that get reused during the lifetime of the program.
pub struct State {
    /// The time at which the program started. Used for telling how much time
    /// has elapsed.
    pub start_time: Instant,
    /// The directory where this process was started from.
    pub current_dir: PathBuf,
    /// The user's home directory.
    pub home_dir: PathBuf,
}

impl State {
    /// Creates a new instance.
    pub fn new() -> crate::Result<Self> {
        let start_time = Instant::now();
        let current_dir = env::current_dir()
            .context("Could not get current working directory")?;
        let home_dir = dirs::home_dir()
            .ok_or(failure::err_msg("Could not get user home directory"))?;
        Ok(Self {
            start_time,
            current_dir,
            home_dir,
        })
    }

    /// Returns the path for `$HOME/.ocean`.
    pub fn home_ocean_dir(&self) -> PathBuf {
        self.home_dir.join(".ocean")
    }

    /// Returns the path for `$HOME/.ocean/credentials.toml`.
    pub fn credentials_path(&self) -> PathBuf {
        let mut path = self.home_ocean_dir();
        path.push("credentials.toml");
        path
    }

    /// Returns the directory where binaries exposed via `$PATH` are stored.
    ///
    /// | Platform | Path               |
    /// | :------- | :----------------- |
    /// | macOS    | "$HOME/.ocean/bin" |
    /// | Linux    | "$HOME/.ocean/bin" |
    /// | Windows  | _Unimplemented_    |
    pub fn bin_dir(&self) -> PathBuf {
        #[cfg(unix)]
        {
            let mut path = self.home_ocean_dir();
            path.push("bin");
            path
        }

        #[cfg(windows)]
        unimplemented!("TODO: Write & test on Windows :)");
    }

    /// Returns Ocean's cache directory.
    pub fn cache_dir(&self) -> PathBuf {
        let mut path = self.home_ocean_dir();
        path.push("cache");
        path
    }

    /// Returns the path where a tarball for `query` should be cached.
    pub fn tarball_cache_path(&self, query: Query<&str>) -> PathBuf {
        let mut path = self.cache_dir();
        path.push(query.tarball_name());
        path
    }

    /// Returns the directory where drops are installed.
    pub fn drops_dir(
        &self,
        target: &InstallTarget,
    ) -> Cow<'static, Path> {
        #[cfg(unix)]
        match target {
            InstallTarget::CurrentUser => {
                let mut home = self.home_ocean_dir();
                home.push("drops");
                Cow::Owned(home)
            },
            InstallTarget::SpecificUser(username) => {
                unimplemented!("TODO: Get base directory for {:?}", username);
            },
            InstallTarget::Global => {
                // TODO+SUDO: Needs admin access to write to either. Should be
                // in a separate process that runs based on user password input.
                // Essentially the same UX as when shells try to run something
                // prefixed with `sudo`
                if cfg!(target_os = "macos") {
                    Cow::Borrowed("/Library/Ocean/drops".as_ref())
                } else {
                    Cow::Borrowed("/usr/local/Ocean/drops".as_ref())
                }
            },
        }

        #[cfg(windows)]
        unimplemented!("TODO: Write & test on Windows :)");
    }
}
