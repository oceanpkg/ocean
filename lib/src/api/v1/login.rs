use std::fmt;
use crate::api;

/// Credentials used for API login.
#[derive(Clone, Copy, Serialize)]
#[non_exhaustive]
pub enum Credentials<S> {
    /// Basic username/password authentication.
    BasicAuth {
        /// The user's account name.
        username: S,
        /// The user's password. This should _never_ be logged or displayed.
        password: S,
    },
}

impl<S> Credentials<S> {
    /// Creates a basic username/password authentication.
    pub const fn basic_auth(username: S, password: S) -> Self {
        Credentials::BasicAuth { username, password }
    }
}

// Required in order to not accidentally log private or personally-identifiable
// information when logging.
impl<S: fmt::Debug> fmt::Debug for Credentials<S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Credentials::BasicAuth { .. } => {
                f.debug_struct("BasicAuth")
                    // Personally identifying information
                    .field("username", &"[pii]")
                    .field("password", &"[private]")
                    .finish()
            },
        }
    }
}

/// Requests an API login token from [`url`].
///
/// This appends the `/v1/login` endpoint to the URL.
///
/// [`url`]: fn.url.html
pub fn request_login_token<S: fmt::Display>(
    credentials: &Credentials<S>,
) -> Result<String, LoginError> {
    let url = api::url()?;
    request_login_token_at(&url, credentials)
}

/// Requests an API login token from a base API URL.
///
/// This mainly exists so that we can also issue requests to testing and staging
/// environments.
pub fn request_login_token_at<S: fmt::Display>(
    api_url: &url::Url,
    credentials: &Credentials<S>,
) -> Result<String, LoginError> {
    let url = api_url.join("/v1/login")?;
    request_login_token_at_specific(url.as_str(), credentials)
}

/// Requests an API login token from a specific URL.
pub fn request_login_token_at_specific<U, S>(
    url: U,
    credentials: &Credentials<S>,
) -> Result<String, LoginError>
where
    U: reqwest::IntoUrl + fmt::Debug,
    S: fmt::Display,
{
    // Monomorphized body to slightly reduce the instruction count of the
    // binary.
    fn request_token(
            builder: reqwest::RequestBuilder,
    ) -> Result<String, LoginError> {
        let response = builder.send()?;

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

    let mut builder = reqwest::Client::new().post(url);

    match credentials {
        Credentials::BasicAuth { username, password } => {
            builder = builder.basic_auth(username, Some(password));
        },
    }

    request_token(builder)
}

/// An error returned when attempting to log into Ocean's API.
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

impl From<url::ParseError> for LoginError {
    fn from(error: url::ParseError) -> Self {
        Self::ParseUrl(error)
    }
}

impl From<reqwest::Error> for LoginError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<http::StatusCode> for LoginError {
    fn from(error: http::StatusCode) -> Self {
        if error == http::StatusCode::UNAUTHORIZED {
            Self::Unauthorized
        } else {
            Self::Status(error)
        }
    }
}

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

impl std::error::Error for LoginError {}
