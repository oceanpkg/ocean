//! Runtime configuration data.

use crate::{drop::name::Query, install::InstallTarget};
use lazycell::LazyCell;
use std::{
    borrow::Cow,
    env,
    error::Error,
    ffi::{OsStr, OsString},
    fmt, io,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

/// Represents the configuration specific to this current instance.
///
/// This type exists so as to put creation of commonly-needed values up-front,
/// allowing for errors only needed to be handled in one place.
#[derive(Clone, Debug)]
pub struct RtConfig {
    /// The time at which the program started. Used for telling how much time
    /// has elapsed.
    pub start_time: Instant,
    /// The directory where this process was started from.
    pub current_dir: PathBuf,
    /// The process's executable.
    pub current_exe: LazyCell<PathBuf>,
    /// The current user's home directory.
    pub user_home: PathBuf,
    /// The directory for data stored for the current user.
    ///
    /// | Platform      | Path            |
    /// | :------------ | :-------------- |
    /// | Linux & macOS | `$HOME/.ocean`  |
    /// | Windows       | _Unimplemented_ |
    pub ocean_home: LazyCell<PathBuf>,
    /// The `PATH` environment variable.
    pub path_var: LazyCell<OsString>,
}

impl RtConfig {
    /// Creates a new instance suitable for using at the start of your program.
    #[inline]
    pub fn create() -> Result<Self, CreateError> {
        Self::create_at(Instant::now())
    }

    /// Creates a new instance suitable for emitting metrics from `start_time`.
    pub fn create_at(start_time: Instant) -> Result<Self, CreateError> {
        Ok(Self {
            start_time,
            current_dir: env::current_dir()
                .map_err(CreateError::MissingCurrentDir)?,
            current_exe: LazyCell::new(),
            user_home: dirs::home_dir()
                .ok_or_else(|| CreateError::MissingUserHome)?,
            ocean_home: LazyCell::new(),
            path_var: LazyCell::new(),
        })
    }

    /// Returns the amount of time elapsed since the program started.
    #[inline]
    pub fn time_elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// The directory where this process was started from.
    #[inline]
    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    /// The process's executable.
    #[inline]
    pub fn current_exe(&self) -> Result<&Path, MissingCurrentExe> {
        fn from_env() -> io::Result<PathBuf> {
            env::current_exe()?.canonicalize()
        }

        fn from_argv(config: &RtConfig) -> Result<PathBuf, MissingCurrentExe> {
            let argv0 = env::args_os()
                .map(PathBuf::from)
                .next()
                .ok_or(MissingCurrentExe(()))?;

            crate::path::resolve_exe(&argv0, config.path_var().ok())
                .map_err(|_| MissingCurrentExe(()))
        }

        self.current_exe
            .try_borrow_with(|| {
                if let Ok(exe) = from_env() {
                    Ok(exe)
                } else {
                    from_argv(self)
                }
            })
            .map(AsRef::as_ref)
    }

    /// The directory where data for the current user is stored.
    pub fn ocean_home(&self) -> &Path {
        self.ocean_home
            .borrow_with(|| self.user_home().join(".ocean"))
    }

    /// The current user's home directory.
    #[inline]
    pub fn user_home(&self) -> &Path {
        &self.user_home
    }

    /// The `PATH` environment variable.
    #[inline]
    pub fn path_var(&self) -> Result<&OsStr, MissingPathVar> {
        self.path_var
            .try_borrow_with(|| env::var_os("PATH").ok_or(MissingPathVar(())))
            .map(AsRef::as_ref)
    }

    /// Returns the path for `$HOME/.ocean/credentials.toml`.
    pub fn credentials_path(&self) -> PathBuf {
        self.ocean_home().join("credentials.toml")
    }

    /// Returns the directory where binaries exposed via `$PATH` are stored.
    pub fn bin_dir(&self) -> PathBuf {
        #[cfg(unix)]
        {
            self.ocean_home().join("bin")
        }

        #[cfg(windows)]
        unimplemented!("TODO: Write & test on Windows :)");
    }

    /// Returns Ocean's cache directory.
    pub fn cache_dir(&self) -> PathBuf {
        self.ocean_home().join("cache")
    }

    /// Returns the path where a tarball for `query` should be cached.
    pub fn tarball_cache_path(&self, query: Query<&str>) -> PathBuf {
        let mut path = self.cache_dir();
        path.push(query.tarball_name());
        path
    }

    /// Returns the directory where drops are installed.
    pub fn drops_dir(&self, target: &InstallTarget) -> Cow<'static, Path> {
        #[cfg(unix)]
        match target {
            InstallTarget::CurrentUser => {
                Cow::Owned(self.ocean_home().join("drops"))
            }
            InstallTarget::SpecificUser(username) => {
                unimplemented!("TODO: Get base directory for {:?}", username);
            }
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
            }
        }

        #[cfg(windows)]
        unimplemented!("TODO: Write & test on Windows :)");
    }
}

/// Indicates [`RtConfig::config`] failed.
///
/// [`RtConfig::config`]: struct.RtConfig.html#method.config
#[derive(Debug)]
pub enum CreateError {
    /// Indicates `env::current_dir` failed.
    MissingCurrentDir(io::Error),
    /// Indicates `dirs::home_dir` failed.
    MissingUserHome,
}

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingCurrentDir(error) => match error.kind() {
                io::ErrorKind::NotFound => {
                    write!(f, "Current directory does not exist")
                }
                io::ErrorKind::PermissionDenied => write!(
                    f,
                    "Not enough permissions to access current directory",
                ),
                _ => write!(f, "Could not get current directory: {}", error),
            },
            Self::MissingUserHome => {
                write!(f, "Could not get current user's home")
            }
        }
    }
}

impl Error for CreateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MissingCurrentDir(error) => Some(error),
            Self::MissingUserHome => None,
        }
    }
}

/// An error indicating that the `PATH` environment variable was not found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MissingPathVar(());

impl fmt::Display for MissingPathVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "missing 'PATH' environment variable")
    }
}

impl Error for MissingPathVar {}

/// An error indicating that the current executable was not found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MissingCurrentExe(());

impl fmt::Display for MissingCurrentExe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot find `ocean` executable")
    }
}

impl Error for MissingCurrentExe {}
