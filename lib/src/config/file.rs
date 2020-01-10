//! Utilities for configuration files.

#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use shared::ext::BytesExt;
use std::{
    error::Error,
    fmt,
    fs::File,
    io,
    path::{Path, PathBuf},
};

const STEM: &str = "ocean";
const STEM_LEN: usize = 5;

mod ext {
    pub const TOML: &str = "toml";
    pub const JSON: &str = "json";
    pub const YAML: &str = "yaml";
    pub const YML: &str = "yml";

    pub const MIN_LEN: usize = 3;
    pub const MAX_LEN: usize = 4;
}

/// Information for the configuration file.
///
/// The configuration file always has a stem of "ocean" (case-insensitive).
#[derive(Debug)]
pub struct ConfigFile {
    /// The location of the configuration file.
    pub path: PathBuf,
    /// The format with which to parse the file.
    pub fmt: ConfigFileFmt,
    /// An open handle to the file at `path`.
    pub handle: Option<File>,
}

impl ConfigFile {
    /// Locates the configuration file at `path`.
    pub fn find(path: &Path) -> Result<Self, NotFound<'_>> {
        const MIN_LEN: usize = STEM_LEN + 1 + ext::MIN_LEN;
        const MAX_LEN: usize = STEM_LEN + 1 + ext::MAX_LEN;

        // Converts `io::Result<T>` to `Result<T, NotFound>`; needed for
        // `path`
        macro_rules! convert_result {
            ($result:expr) => {
                $result.map_err(|err| {
                    let reason = NotFoundReason::Io(err);
                    NotFound { reason, path }
                })
            };
        }

        for entry in convert_result!(path.read_dir())? {
            let entry = convert_result!(entry)?;
            let name = entry.file_name();

            // Only do cheap checks
            match name.len() {
                MIN_LEN..=MAX_LEN => {}
                _ => continue,
            }

            #[cfg(unix)]
            let bytes = name.as_bytes();

            // Needed since bytes can only be retrieved via `str` on non-Unix :(
            #[cfg(not(unix))]
            let bytes = if let Some(s) = name.to_str() {
                s.as_bytes()
            } else {
                continue;
            };

            // SAFETY: We call `.get_unchecked()` because the optimizer
            // apparently doesn't know that `bytes.len() == name.len()`
            let stem = unsafe { bytes.get_unchecked(..STEM_LEN) };
            if !stem.matches_special_lowercase(STEM) {
                continue;
            }

            // SAFETY: See above^
            if unsafe { *bytes.get_unchecked(STEM_LEN) } != b'.' {
                continue;
            }

            // SAFETY: See above^
            let ext = unsafe { bytes.get_unchecked((STEM_LEN + 1)..) };

            if let Some(fmt) = ConfigFileFmt::from_bytes(ext) {
                return Ok(ConfigFile {
                    path: path.join(name),
                    fmt,
                    handle: None,
                });
            } else {
                continue;
            };
        }

        Err(NotFound {
            reason: NotFoundReason::NoMatch,
            path,
        })
    }
}

/// The format of the configuration file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfigFileFmt {
    /// [Tom's Obvious, Minimal Language](https://github.com/toml-lang/toml).
    ///
    /// Extension: `toml`
    Toml,
    /// [JavaScript Object Notation](https://json.org).
    ///
    /// Extension: `json`
    Json,
    /// [YAML Ain't Markup Language](http://yaml.org).
    ///
    /// Extensions: `yaml`, `yml`
    Yaml,
}

impl ConfigFileFmt {
    /// Returns the corresponding variant for the file extension, going from
    /// TOML to JSON to YAML in order.
    ///
    /// This expects `bytes` to be ASCII/UTF-8 and will simply fail with other
    /// encodings.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Perform a case-insensitive match on each extension and assign the
        // corresponding `ConfigFileFmt` variant, moving onto the next entry if
        // none match
        macro_rules! handle_ext {
            ($($fmt:ident => $($ext:expr),+;)+) => {
                $(if $(bytes.matches_special_lowercase($ext))||+ {
                    Some(ConfigFileFmt::$fmt)
                } else)+ {
                    None
                }
            };
        }

        handle_ext! {
            Toml => ext::TOML;
            Json => ext::JSON;
            Yaml => ext::YAML, ext::YML;
        }
    }

    /// Returns the corresponding variant for the file pointed to at `path`
    /// based on its extension, going from TOML to JSON to YAML in order.
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?;
        match ext.len() {
            // Range allows for doing only cheap UTF-8 checks on non-Unix
            ext::MIN_LEN..=ext::MAX_LEN => {
                // Assume ASCII extension
                #[cfg(unix)]
                let ext = ext.as_bytes();

                #[cfg(not(unix))]
                let ext = ext.to_str()?.as_bytes();

                ConfigFileFmt::from_bytes(ext)
            }
            _ => None,
        }
    }
}

/// The underlying cause for `ConfigFile::find` to return
/// [`NotFound`](struct.NotFound.html).
#[derive(Debug)]
pub enum NotFoundReason {
    /// Could not read a directory or entries within the directory.
    Io(io::Error),
    /// The Ocean configuration file could not be found in the directory it's
    /// expected to be in.
    NoMatch,
}

impl From<io::Error> for NotFoundReason {
    fn from(error: io::Error) -> Self {
        NotFoundReason::Io(error)
    }
}

/// The error returned by `ConfigFile::find`.
#[derive(Debug)]
pub struct NotFound<'a> {
    /// The underlying cause.
    pub reason: NotFoundReason,
    /// The path being searched when the error ocurred.
    pub path: &'a Path,
}

impl Error for NotFound<'_> {}

impl fmt::Display for NotFound<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.reason {
            NotFoundReason::Io(err) => write!(f, "{} for {:?}", err, self.path),
            NotFoundReason::NoMatch => write!(
                f,
                "No TOML, JSON, or YAML file named \"ocean\" found in \"{}\"",
                self.path.display()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use self::ConfigFileFmt::*;
    use super::*;

    static PAIRS: &[(ConfigFileFmt, &[&str])] = &[
        (Toml, &[ext::TOML]),
        (Json, &[ext::JSON]),
        (Yaml, &[ext::YAML, ext::YML]),
    ];

    #[test]
    fn find_cfg_file() {
        let dir = tempfile::tempdir().unwrap();

        match ConfigFile::find(dir.path()) {
            Ok(file) => panic!("Found unexpected config {:?}", file),
            Err(err) => match err.reason {
                NotFoundReason::NoMatch => {}
                NotFoundReason::Io(err) => panic!("{}", err),
            },
        }

        for &(fmt, exts) in PAIRS {
            for &ext in exts {
                let cfg_name = format!("{}.{}", STEM, ext);
                let upper = cfg_name.to_uppercase();
                let lower = cfg_name.to_lowercase();

                for cfg_name in &[lower, upper] {
                    let cfg_path = dir.path().join(&cfg_name);
                    std::fs::File::create(&cfg_path).unwrap();

                    let cfg_file = ConfigFile::find(dir.path()).unwrap();
                    assert_eq!(cfg_file.path, cfg_path);
                    assert_eq!(cfg_file.fmt, fmt);

                    std::fs::remove_file(cfg_path).unwrap();
                }
            }
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;

            let no_read = dir.path().join("no_read");
            std::fs::DirBuilder::new()
                .mode(0) // no permissions
                .create(&no_read)
                .unwrap();

            let exp_err = std::io::ErrorKind::PermissionDenied;
            match ConfigFile::find(&no_read) {
                Ok(file) => panic!("Found unexpected config {:?}", file),
                Err(err) => match err.reason {
                    NotFoundReason::NoMatch => panic!("Should emit IO error"),
                    NotFoundReason::Io(err) => assert_eq!(err.kind(), exp_err),
                },
            }
        }
    }

    #[test]
    fn fmt_from_path() {
        let prefixes: &[_] = &["", "/", "./", "/xyz/"];

        for &(fmt, exts) in PAIRS {
            for ext in exts {
                for prefix in prefixes {
                    let path =
                        PathBuf::from(format!("{}{}.{}", prefix, STEM, ext));
                    assert_eq!(ConfigFileFmt::from_path(&path).unwrap(), fmt);
                }
            }
        }
    }
}
