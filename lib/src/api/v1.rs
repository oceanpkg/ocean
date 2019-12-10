//! Version 1 of Ocean's web API.

/// The default URL to which API calls are made: `https://api.oceanpkg.org/v1/`.
pub const DEFAULT_URL: &str = "https://api.oceanpkg.org/v1/";

/// Requests an API login token from [`url`].
///
/// This appends the `/v1/login` endpoint to the URL.
///
/// [`url`]: fn.url.html
#[cfg(feature = "reqwest")]
pub fn request_login_token(
    username: &str,
    password: &str,
) -> Result<String, LoginError> {
    let url = super::url()?;
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
) -> Result<String, LoginError> {
    let url = api_url.join("/v1/login")?;
    request_login_token_at_specific(url.as_str(), username, password)
}

/// Requests an API login token from a specific URL.
#[cfg(feature = "reqwest")]
pub fn request_login_token_at_specific<U: reqwest::IntoUrl>(
    url: U,
    username: &str,
    password: &str,
) -> Result<String, LoginError> {
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
    ) -> Result<String, LoginError> {
        let response = builder
            .json(credentials)
            .send()?;

        let status = response.status();
        if !status.is_success() {
            return Err(LoginError::from(status));
        }

        for cookie in response.cookies() {
            if cookie.name() == "token" {
                return Ok(cookie.value().to_owned());
            }
        }

        Err(LoginError::MissingToken)
    }

    request_token(
        reqwest::Client::new().post(url),
        &LoginCredentials { username, password },
    )
}

/// An error returned when attempting to log into Ocean's API.
#[cfg(feature = "reqwest")]
#[derive(Debug)]
pub enum LoginError {
    /// Failed to parse a `Url`.
    ParseUrl(url::ParseError),
    /// Failed to send the request via `reqwest`.
    Request(reqwest::Error),
    /// Received an error status code.
    Status(http::StatusCode),
    /// Failed to authenticate (401 status).
    Unauthorized,
    /// Received a successful response, but no token was provided.
    MissingToken,
}

#[cfg(feature = "reqwest")]
impl From<url::ParseError> for LoginError {
    fn from(error: url::ParseError) -> Self {
        Self::ParseUrl(error)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for LoginError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

#[cfg(feature = "reqwest")]
impl From<http::StatusCode> for LoginError {
    fn from(error: http::StatusCode) -> Self {
        if error == http::StatusCode::UNAUTHORIZED {
            Self::Unauthorized
        } else {
            Self::Status(error)
        }
    }
}

#[cfg(feature = "reqwest")]
impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::LoginError::*;
        match self {
            ParseUrl(error) => error.fmt(f),
            Request(error) => error.fmt(f),
            Status(code) => write!(f, "received response \"{}\"", code),
            Unauthorized => write!(f, "incorrect username or password"),
            MissingToken => write!(f, "login token was not received"),
        }
    }
}

#[cfg(feature = "reqwest")]
impl std::error::Error for LoginError {}
