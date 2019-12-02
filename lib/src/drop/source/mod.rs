//! The source of a package.

use url::Url;

pub mod git;

pub use self::git::Git;

use self::git::Ref;

const OCEAN_REGISTRY: &str = "https://registry.oceanpkg.org";

lazy_static! {
    static ref OCEAN_REGISTRY_SOURCE: Source = Source::from_registry(
        Url::parse(OCEAN_REGISTRY).unwrap()
    );
}

/// The source of a drop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    url: Url,
    kind: Kind,
}

impl Source {
    /// A drop source for the main Ocean registry.
    #[inline]
    pub fn main_registry() -> &'static Self {
        &OCEAN_REGISTRY_SOURCE
    }

    /// A drop source at a `Url` for an Ocean registry.
    #[inline]
    pub const fn from_registry(url: Url) -> Self {
        Source { url, kind: Kind::Registry }
    }

    /// A drop source at a `Url` for to a git repository.
    #[inline]
    pub fn from_git(url: Url) -> Self {
        Self::from_git_at(url, Ref::master())
    }

    /// A drop source at a `Url` for a git repository at a specific reference.
    #[inline]
    pub const fn from_git_at(url: Url, reference: Ref) -> Self {
        Source { url, kind: Kind::Git(reference) }
    }

    /// Where this source is located.
    #[inline]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// The type of source.
    #[inline]
    pub const fn kind(&self) -> &Kind {
        &self.kind
    }
}

/// Determines how to treat a [`Source`](struct.Source.html).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The drop is located in a git repository and the given reference should
    /// be used.
    Git(Ref),
    /// The drop is located in a registry.
    Registry,
}
