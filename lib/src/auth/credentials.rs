//! User credentials.

/// User credentials.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Credentials<S = String> {
    /// Credentials for the main Ocean registry.
    pub registry: Option<Registry<S>>,
}

/// Credentials for the main Ocean registry.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Registry<S> {
    /// A token associated with a specific user that provides permissions for
    /// interacting with packages in a registry.
    pub token: S,
}
