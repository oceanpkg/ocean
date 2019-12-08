//! Interfacing with Ocean's web API.

use std::{
    borrow::Cow,
    env::{self, VarError},
    ffi::{OsStr, OsString},
};
use url::Url;

pub mod v1;

/// The default URL to which API calls are made: `https://api.oceanpkg.org/`.
pub const DEFAULT_URL: &str = "https://api.oceanpkg.org/";

/// The environment variable key for using an alternative API URL:
/// `OCEAN_API_URL`.
pub const URL_ENV_KEY: &str = crate::env::OCEAN_API_URL;

/// Returns the parsed `Url` value for [`URL_ENV_VAR`] or [`DEFAULT_URL`] if it
/// does not exist.
///
/// [`URL_ENV_VAR`]: constant.URL_ENV_VAR.html
/// [`DEFAULT_URL`]: constant.DEFAULT_URL.html
pub fn url() -> Result<Url, url::ParseError> {
    match url_str() {
        Ok(url) => Url::parse(&url),
        Err(_) => Err(url::ParseError::InvalidDomainCharacter),
    }
}

/// Returns the UTF-8 encoded value for [`URL_ENV_VAR`] or [`DEFAULT_URL`] if it
/// does not exist.
///
/// [`URL_ENV_VAR`]: constant.URL_ENV_VAR.html
/// [`DEFAULT_URL`]: constant.DEFAULT_URL.html
pub fn url_str() -> Result<Cow<'static, str>, OsString> {
    match env::var(URL_ENV_KEY) {
        Ok(var) => Ok(Cow::Owned(var)),
        Err(VarError::NotPresent) => Ok(Cow::Borrowed(DEFAULT_URL)),
        Err(VarError::NotUnicode(var)) => Err(var),
    }
}

/// Returns the OS encoded value for [`URL_ENV_VAR`] or [`DEFAULT_URL`] if it
/// does not exist.
///
/// [`URL_ENV_VAR`]: constant.URL_ENV_VAR.html
/// [`DEFAULT_URL`]: constant.DEFAULT_URL.html
pub fn url_os_str() -> Cow<'static, OsStr> {
    if let Some(var) = env::var_os(URL_ENV_KEY) {
        Cow::Owned(var)
    } else {
        Cow::Borrowed(DEFAULT_URL.as_ref())
    }
}
