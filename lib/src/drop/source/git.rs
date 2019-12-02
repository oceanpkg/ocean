//! Git repository information.

use std::fmt;
use serde::{Serialize, Serializer};

flexible! {
    /// Information about a git repository where a drop or dependency can be found.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
    pub struct Git<'a> {
        /// Where the git repository is located.
        #[serde(alias = "repository")]
        pub repo: &'a str,
        /// The specific branch to use.
        #[serde(flatten)]
        pub reference: Option<Ref<'a>>,
    }
}

impl<'a> From<&'a str> for Git<'a> {
    #[inline]
    fn from(repo: &'a str) -> Self {
        Self { repo, reference: None }
    }
}

impl<'a> Git<'a> {
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
    pub fn display_toml(&self) -> impl fmt::Display + Copy + 'a {
        #[derive(Clone, Copy)]
        struct Displayer<'a>(Git<'a>);

        impl fmt::Display for Displayer<'_> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.0.write_toml(f)
            }
        }

        Displayer(*self)
    }
}

/// A reference to a git branch/tag/revision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Ref<'a> {
    // When adding a case, make sure to add it to `Ref::all`.

    /// The specific git branch.
    Branch(&'a str),
    /// A specific git tag.
    Tag(&'a str),
    /// A specific git revision.
    Rev(&'a str),
}

impl Default for Ref<'_> {
    #[inline]
    fn default() -> Self {
        Self::MASTER
    }
}

impl fmt::Display for Ref<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for Ref<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Serialize for Ref<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeMap;

        let mut map = ser.serialize_map(Some(1))?;
        map.serialize_entry(self.kind(), self.as_str())?;
        map.end()
    }
}

impl<'a> Ref<'a> {
    #[cfg(test)]
    pub(crate) const fn all(reference: &'a str) -> [Self; 3] {
        [
            Ref::Branch(reference),
            Ref::Tag(reference),
            Ref::Rev(reference),
        ]
    }

    #[cfg(test)]
    pub(crate) const TEST_ALL: [Self; 3] = Self::all(
        // Commit hashes are 40 characters long.
        "0000111122223333444455556666777788889999"
    );

    /// A reference to the master branch.
    pub const MASTER: Self = Ref::Branch("master");

    /// Returns the reference string.
    #[inline]
    pub fn as_str(&self) -> &'a str {
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

#[cfg(test)]
mod tests {
    use super::*;

    const OCEAN_REPO: &str = env!("CARGO_PKG_REPOSITORY");

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Parsed<'a> {
        #[serde(borrow)]
        git: Git<'a>,
    }

    impl<'a> Parsed<'a> {
        fn parse(toml: &'a str) -> Self {
            toml::de::from_str::<Parsed>(toml).unwrap()
        }
    }

    #[test]
    fn deserialize_toml_repo() {
        let parsed = Parsed::parse(r#"
            git = "https://github.com/oceanpkg/ocean.git"
        "#);
        let expected = Parsed {
            git: "https://github.com/oceanpkg/ocean.git".into()
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn deserialize_toml_detailed() {
        let parsed = Parsed::parse(r#"
            [git]
            repo = "https://github.com/oceanpkg/ocean.git"
            tag = "lib-v0.0.8"
        "#);
        let expected = Parsed {
            git: Git {
                repo: "https://github.com/oceanpkg/ocean.git",
                reference: Some(Ref::Tag("lib-v0.0.8")),
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn deserialize_toml_detailed_table() {
        let parsed = Parsed::parse(r#"
            git = { repo = "https://github.com/oceanpkg/ocean.git", tag = "lib-v0.0.8" }
        "#);
        let expected = Parsed {
            git: Git {
                repo: "https://github.com/oceanpkg/ocean.git",
                reference: Some(Ref::Tag("lib-v0.0.8")),
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn serialize_toml_checkout() {
        for reference in Ref::TEST_ALL.iter() {
            toml::to_string(reference).unwrap();
            toml::to_string_pretty(reference).unwrap();
        }
    }

    #[test]
    fn serialize_toml_git() {
        use std::iter;

        // Creates `Option::Some` cases for known reference types and a `None`
        // case to be thorough.
        let checkouts = Ref::TEST_ALL.iter()
            .cloned()
            .map(Some)
            .chain(iter::once(None));

        for reference in checkouts {
            let git = Git {
                repo: OCEAN_REPO,
                reference,
            };
            toml::to_string(&git).unwrap();
            toml::to_string_pretty(&git).unwrap();
        }
    }
}
