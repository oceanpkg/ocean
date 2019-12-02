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
        pub checkout: Option<Checkout<'a>>,
    }
}

impl<'a> From<&'a str> for Git<'a> {
    #[inline]
    fn from(repo: &'a str) -> Self {
        Self { repo, checkout: None }
    }
}

impl<'a> Git<'a> {
    /// Writes the TOML form of `self` to `f`.
    #[inline]
    pub fn write_toml(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"git = {{ repo = "{}""#, self.repo)?;
        if let Some(checkout) = &self.checkout {
            write!(f, r#", {} = "{}""#, checkout.kind(), checkout)?;
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

/// The git checkout to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Checkout<'a> {
    // When adding a case, make sure to add it to `Checkout::all`.

    /// The specific git branch.
    Branch(&'a str),
    /// A specific git tag.
    Tag(&'a str),
    /// A specific git revision.
    Rev(&'a str),
}

impl Default for Checkout<'_> {
    #[inline]
    fn default() -> Self {
        Self::MASTER
    }
}

impl fmt::Display for Checkout<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for Checkout<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Serialize for Checkout<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeMap;

        let mut map = ser.serialize_map(Some(1))?;
        map.serialize_entry(self.kind(), self.as_str())?;
        map.end()
    }
}

impl<'a> Checkout<'a> {
    #[cfg(test)]
    pub(crate) const fn all(checkout: &'a str) -> [Self; 3] {
        [
            Checkout::Branch(checkout),
            Checkout::Tag(checkout),
            Checkout::Rev(checkout),
        ]
    }

    #[cfg(test)]
    pub(crate) const TEST_ALL: [Self; 3] = Self::all(
        // Commit hashes are 40 characters long.
        "0000111122223333444455556666777788889999"
    );

    /// A reference to the master branch.
    pub const MASTER: Self = Checkout::Branch("master");

    /// Returns the checkout string.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        match self {
            Self::Branch(ch) |
            Self::Tag(ch) |
            Self::Rev(ch) => ch
        }
    }

    /// Returns the name of the checkout kind: `branch`, `tag`, or `rev`.
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
                checkout: Some(Checkout::Tag("lib-v0.0.8")),
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
                checkout: Some(Checkout::Tag("lib-v0.0.8")),
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn serialize_toml_checkout() {
        for checkout in Checkout::TEST_ALL.iter() {
            toml::to_string(checkout).unwrap();
            toml::to_string_pretty(checkout).unwrap();
        }
    }

    #[test]
    fn serialize_toml_git() {
        use std::iter;

        // Creates `Option::Some` cases for known checkout types and a `None`
        // case to be thorough.
        let checkouts = Checkout::TEST_ALL.iter()
            .cloned()
            .map(Some)
            .chain(iter::once(None));

        for checkout in checkouts {
            let git = Git {
                repo: OCEAN_REPO,
                checkout,
            };
            toml::to_string(&git).unwrap();
            toml::to_string_pretty(&git).unwrap();
        }
    }
}
