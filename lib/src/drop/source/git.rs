//! Git repository information.

use std::fmt;
use serde::{Serialize, Serializer};

/// Ocean's git repository.
pub const OCEAN_REPO: &str = env!("CARGO_PKG_REPOSITORY");

flexible! {
    /// Information about a git repository where a drop or dependency can be found.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
    pub struct Git {
        /// Where the git repository is located.
        #[serde(alias = "repository")]
        pub repo: String,
        /// The specific branch to use.
        #[serde(flatten)]
        pub reference: Option<Ref>,
    }
}

impl From<String> for Git {
    #[inline]
    fn from(repo: String) -> Self {
        Self { repo, reference: None }
    }
}

impl Git {
    /// Creates a new instance with the given fields.
    pub fn new<A, B>(repo: A, reference: B) -> Self
    where
        A: Into<String>,
        B: Into<Option<Ref>>,
    {
        Self {
            repo: repo.into(),
            reference: reference.into(),
        }
    }

    /// Writes the TOML form of `self` to `f`.
    #[inline]
    pub fn write_toml(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"git = {{ repo = "{}""#, self.repo)?;
        if let Some(reference) = &self.reference {
            write!(f, r#", {} = "{}""#, reference.kind(), reference)?;
        }
        write!(f, " }}")
    }

    /// Returns a type that can be used to as `{}` to display TOML.
    #[inline]
    pub fn display_toml<'a>(&'a self) -> impl fmt::Display + Copy + 'a {
        #[derive(Clone, Copy)]
        struct Displayer<'a>(&'a Git);

        impl fmt::Display for Displayer<'_> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.write_toml(f)
            }
        }

        Displayer(self)
    }
}

/// A reference to a git branch/tag/revision.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ref {
    // When adding a case, make sure to add it to `Ref::all`.

    /// The specific git branch.
    Branch(String),
    /// A specific git tag.
    Tag(String),
    /// A specific git revision.
    Rev(String),
}

impl Default for Ref {
    #[inline]
    fn default() -> Self {
        Self::master()
    }
}

impl fmt::Display for Ref {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for Ref {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Serialize for Ref {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeMap;

        let mut map = ser.serialize_map(Some(1))?;
        map.serialize_entry(self.kind(), self.as_str())?;
        map.end()
    }
}

impl Ref {
    /// Returns an array of all `Ref` variants, each pointing to `reference`.
    pub fn all(reference: String) -> [Self; 3] {
        [
            Ref::Branch(reference.clone()),
            Ref::Tag(reference.clone()),
            Ref::Rev(reference),
        ]
    }

    /// Creates a new `Branch` instance pointing to `reference`.
    pub fn branch<R: Into<String>>(reference: R) -> Self {
        Ref::Branch(reference.into())
    }

    /// Creates a new `Tag` instance pointing to `reference`.
    pub fn tag<R: Into<String>>(reference: R) -> Self {
        Ref::Tag(reference.into())
    }

    /// Creates a new `Rev` instance pointing to `reference`.
    pub fn rev<R: Into<String>>(reference: R) -> Self {
        Ref::Rev(reference.into())
    }

    /// A reference to the master branch.
    #[inline]
    pub fn master() -> Self {
        Ref::branch("master")
    }

    /// Returns the reference string.
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Branch(r) | Self::Tag(r) | Self::Rev(r) => r
        }
    }

    /// Returns the name of the reference kind: `branch`, `tag`, or `rev`.
    #[inline]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Branch(_) => "branch",
            Self::Tag(_) => "tag",
            Self::Rev(_) => "rev",
        }
    }
}
