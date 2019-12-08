//! Interfacing with Ocean's web API.

use std::{
    borrow::Cow,
    env::{self, VarError},
    ffi::{OsStr, OsString},
};
use url::Url;

#[cfg(feature = "reqwest")]
use std::io;

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

#[cfg(feature = "reqwest")]
fn convert_parse_error(error: url::ParseError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}

/// Requests an API login token from [`url`].
///
/// This appends the `/v1/login` endpoint to the URL.
///
/// [`url`]: fn.url.html
#[cfg(feature = "reqwest")]
pub fn request_login_token(
    username: &str,
    password: &str,
) -> io::Result<String> {
    let url = url().map_err(convert_parse_error)?;
    request_login_token_at(&url, username, password)
}

/// Requests an API login token from a base API URL.
///
/// This mainly exists so that we can also issue requests to testing and staging
/// environments.
#[cfg(feature = "reqwest")]
pub fn request_login_token_at(
    api_url: &Url,
    username: &str,
    password: &str,
) -> io::Result<String> {
    let url = api_url.join("/v1/login").map_err(convert_parse_error)?;
    request_login_token_at_specific(url.as_str(), username, password)
}

/// Requests an API login token from a specific URL.
pub fn request_login_token_at_specific<U: reqwest::IntoUrl>(
    url: U,
    username: &str,
    password: &str,
) -> io::Result<String> {
    #[derive(Serialize)]
    struct LoginCredentials<'a> {
        username: &'a str,
        password: &'a str,
    }

    // Monomorphized body to slightly reduce the instruction count of the
    // binary.
    fn request_token(
            builder: reqwest::RequestBuilder,
            credentials: &LoginCredentials,
    ) -> io::Result<String> {
        let response = builder
            .json(credentials)
            .send()
            .map_err(|error| {
                io::Error::new(io::ErrorKind::Other, error)
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_kind = io::ErrorKind::Other;
            let error = format!("received response \"{}\"", status);
            return Err(io::Error::new(error_kind, error));
        }

        for cookie in response.cookies() {
            if cookie.name() == "token" {
                return Ok(cookie.value().to_owned());
            }
        }

        let error_kind = io::ErrorKind::InvalidData;
        let error = "the API login token was not received";
        return Err(io::Error::new(error_kind, error));
    }

    request_token(
        reqwest::Client::new().post(url),
        &LoginCredentials { username, password },
    )
}
