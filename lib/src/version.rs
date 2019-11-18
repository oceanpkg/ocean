//! Versioning schemes.

/// A drop version.
pub enum Version {
    /// [Semantic versioning](http://semver.org). This is the default.
    SemVer(semver::Version),
    /// A custom versioning scheme.
    Custom(String),
}
