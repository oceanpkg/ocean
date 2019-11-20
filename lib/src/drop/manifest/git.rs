//! Git repository information.

use std::fmt;

/// Information about a git repository where a drop or dependency can be found.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Git<'a> {
    /// Simply point to the repository.
    Repo(&'a str),
    /// Detailed information about the repository.
    Detailed {
        /// Where the git repository is located.
        #[serde(alias = "repository")]
        repo: &'a str,
        /// The specific branch to use.
        #[serde(flatten)]
        checkout: Option<Checkout<'a>>,
    }
}

impl<'a> Git<'a> {
    /// Returns the git repository.
    #[inline]
    pub fn repo(&self) -> &'a str {
        match self {
            Git::Repo(repo) | Git::Detailed { repo, .. } => repo
        }
    }
}

/// The git checkout to use.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Checkout<'a> {
    /// The specific git branch.
    Branch(&'a str),
    /// A specific git tag.
    Tag(&'a str),
    /// A specific git revision.
    Rev(&'a str),
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

impl<'a> Checkout<'a> {
    /// Returns the checkout string.
    #[inline]
    pub fn as_str(&self) -> &'a str {
        match self {
            Checkout::Branch(ch) |
            Checkout::Tag(ch) |
            Checkout::Rev(ch) => ch
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn toml_repo() {
        let parsed = Parsed::parse(r#"
            git = "https://github.com/oceanpkg/ocean.git"
        "#);
        let expected = Parsed {
            git: Git::Repo("https://github.com/oceanpkg/ocean.git"),
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn toml_detailed() {
        let parsed = Parsed::parse(r#"
            [git]
            repo = "https://github.com/oceanpkg/ocean.git"
            tag = "lib-v0.0.4"
        "#);
        let expected = Parsed {
            git: Git::Detailed {
                repo: "https://github.com/oceanpkg/ocean.git",
                checkout: Some(Checkout::Tag("lib-v0.0.4")),
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn toml_detailed_table() {
        let parsed = Parsed::parse(r#"
            git = { repo = "https://github.com/oceanpkg/ocean.git", tag = "lib-v0.0.4" }
        "#);
        let expected = Parsed {
            git: Git::Detailed {
                repo: "https://github.com/oceanpkg/ocean.git",
                checkout: Some(Checkout::Tag("lib-v0.0.4")),
            },
        };
        assert_eq!(parsed, expected);
    }
}
