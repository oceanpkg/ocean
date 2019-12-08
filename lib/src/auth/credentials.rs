//! User credentials.

/// User credentials.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Credentials {
    /// Credentials for the main Ocean registry.
    pub registry: Option<Registry>,
}

/// Credentials for the main Ocean registry.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Registry {
    /// A token associated with a specific user that provides permissions for
    /// interacting with packages in a registry.
    pub token: String,
}
