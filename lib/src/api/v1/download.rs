use crate::{api, drop::name::Query};
use std::io;

/// Requests the archive for a drop that matches `query` from [`url`].
///
/// [`url`]: fn.url.html
pub fn download_drop(
    query: Query<&str>,
    writer: &mut dyn io::Write,
) -> Result<(), DownloadError> {
    let url = api::url()?;
    download_drop_at(&url, query, writer)
}

/// Requests the archive for a drop that matches `query` from a base API URL.
///
/// This mainly exists so that we can also issue requests to testing and staging
/// environments.
pub fn download_drop_at(
    api_url: &url::Url,
    query: Query<&str>,
    writer: &mut dyn io::Write,
) -> Result<(), DownloadError> {
    let url = query.join_to_url(&api_url.join("/v1/")?)?;
    download_drop_at_specific(url.as_str(), writer)
}

/// Requests an API login token from a specific URL.
pub fn download_drop_at_specific<U: reqwest::IntoUrl>(
    url: U,
    writer: &mut dyn io::Write,
) -> Result<(), DownloadError> {
    // Monomorphized body to slightly reduce the instruction count of the
    // binary.
    fn download_drop(
        builder: reqwest::RequestBuilder,
        writer: &mut dyn io::Write,
    ) -> Result<(), DownloadError> {
        let mut response = builder.send()?;

        let status = response.status();
        if !status.is_success() {
            return Err(DownloadError::from(status));
        }

        response.copy_to(writer)?;

        Ok(())
    }

    download_drop(reqwest::Client::new().get(url), writer)
}

/// An error returned when attempting to log into Ocean's API.
#[derive(Debug)]
pub enum DownloadError {
    /// Failed to parse a `Url`.
    ParseUrl(url::ParseError),
    /// Failed to send the request via `reqwest`.
    Request(reqwest::Error),
    /// Received an error status code.
    Status(http::StatusCode),
    /// Failed to write the response.
    Io(io::Error),
}

impl From<url::ParseError> for DownloadError {
    fn from(error: url::ParseError) -> Self {
        Self::ParseUrl(error)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<http::StatusCode> for DownloadError {
    fn from(error: http::StatusCode) -> Self {
        Self::Status(error)
    }
}

impl From<io::Error> for DownloadError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::DownloadError::*;
        match self {
            ParseUrl(error) => error.fmt(f),
            Request(error) => error.fmt(f),
            Status(code) => write!(f, "received response \"{}\"", code),
            Io(error) => write!(f, "failed to write response: {}", error),
        }
    }
}

impl std::error::Error for DownloadError {}
