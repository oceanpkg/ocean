//! Version 1 of Ocean's web API.

#[cfg(feature = "reqwest")]
use std::io;

/// The default URL to which API calls are made: `https://api.oceanpkg.org/v1/`.
pub const DEFAULT_URL: &str = "https://api.oceanpkg.org/v1/";

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
    let url = super::url().map_err(convert_parse_error)?;
    request_login_token_at(&url, username, password)
}

/// Requests an API login token from a base API URL.
///
/// This mainly exists so that we can also issue requests to testing and staging
/// environments.
#[cfg(feature = "reqwest")]
pub fn request_login_token_at(
    api_url: &url::Url,
    username: &str,
    password: &str,
) -> io::Result<String> {
    let url = api_url.join("/v1/login").map_err(convert_parse_error)?;
    request_login_token_at_specific(url.as_str(), username, password)
}

/// Requests an API login token from a specific URL.
#[cfg(feature = "reqwest")]
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
