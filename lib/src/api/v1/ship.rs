use std::{
    fs::File,
    mem,
};
use reqwest::{
    Client,
    header,
    multipart::{Form, Part},
    RequestBuilder,
};
use crate::{
    api,
    drop::Package,
};

/// Requests an API login token from [`url`].
///
/// This appends the `/v1/login` endpoint to the URL.
///
/// [`url`]: fn.url.html
pub fn ship(
    package: &Package,
    token: &str,
) -> Result<reqwest::Response, ShipError> {
    let url = api::url()?;
    ship_at(&url, package, token)
}

/// Requests an API login token from a base API URL.
///
/// This mainly exists so that we can also issue requests to testing and staging
/// environments.
pub fn ship_at(
    api_url: &url::Url,
    package: &Package,
    token: &str,
) -> Result<reqwest::Response, ShipError> {
    let url = api_url.join("/v1/packages/create")?;
    ship_at_specific(url.as_str(), package, token)
}

/// Requests an API login token from a specific URL.
pub fn ship_at_specific<U: reqwest::IntoUrl>(
    url: U,
    package: &Package,
    token: &str,
) -> Result<reqwest::Response, ShipError> {
    // Monomorphized body to slightly reduce the instruction count of the
    // binary.
    fn ship(
        builder: RequestBuilder,
        package: &Package,
        token: &str,
    ) -> Result<reqwest::Response, ShipError> {
        let version = package.manifest.meta.version.to_string();

        // SAFETY: `Form::text` requires a `'static` lifetime for string slices.
        // This lifetime extension is fine to satisfy this requirement because
        // the reference does not escape this scope.
        let name = unsafe {
            let name = package.manifest.meta.name.as_str();
            mem::transmute::<&str, &'static str>(name)
        };

        let form = Form::new()
            .text("name", name)
            .text("version", version);

        // SAFETY: `Part::reader` requires a `'static` lifetime. This lifetime
        // extension for `package.file` is fine to satisfy this requirement
        // because the reference does not escape this scope. This is enforced
        // by shadowing the reference's name, making it impossible to use later.
        let package = unsafe {
            mem::transmute::<&File, &'static File>(&package.file)
        };
        let package = Part::reader(package)
            .mime_str("application/gzip")?
            .file_name("packageFile");
        let form = form.part("packageFile", package);

        let response = builder.multipart(form)
            .header(header::COOKIE, format!("token={}", token))
            .send()?;

        // TODO: Change to `debug!`
        eprintln!("Received response: {:#?}", response);

        let status = response.status();
        if !status.is_success() {
            return Err(ShipError::from(status));
        }

        Ok(response)
    }

    ship(Client::new().post(url), package, token)
}

/// An error returned when attempting to ship a package with Ocean's API.
#[derive(Debug)]
pub enum ShipError {
    /// Failed to parse a `Url`.
    ParseUrl(url::ParseError),
    /// Failed to serialize the manifest as JSON.
    SerializeManifest(serde_json::Error),
    /// Failed to send the request via `reqwest`.
    Request(reqwest::Error),
    /// Received an error status code.
    Status(http::StatusCode),
    /// Failed to authenticate (401 status).
    Unauthorized,
}

impl From<url::ParseError> for ShipError {
    fn from(error: url::ParseError) -> Self {
        Self::ParseUrl(error)
    }
}

impl From<serde_json::Error> for ShipError {
    fn from(error: serde_json::Error) -> Self {
        Self::SerializeManifest(error)
    }
}

impl From<reqwest::Error> for ShipError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error)
    }
}

impl From<http::StatusCode> for ShipError {
    fn from(error: http::StatusCode) -> Self {
        if error == http::StatusCode::UNAUTHORIZED {
            Self::Unauthorized
        } else {
            Self::Status(error)
        }
    }
}

impl std::fmt::Display for ShipError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::ShipError::*;
        match self {
            ParseUrl(error) => error.fmt(f),
            SerializeManifest(error) => error.fmt(f),
            Request(error) => error.fmt(f),
            Status(code) => write!(f, "received response \"{}\"", code),
            Unauthorized => write!(f, "incorrect username or password"),
        }
    }
}

impl std::error::Error for ShipError {}
