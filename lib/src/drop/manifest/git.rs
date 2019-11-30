//! Git repository information.

use std::fmt;

/// Information about a git repository where a drop or dependency can be found.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Git {
    /// Simply point to the repository.
    Repo(String),
    /// Detailed information about the repository.
    Detailed {
        /// Where the git repository is located.
        #[serde(alias = "repository")]
        repo: String,
        /// The specific branch to use.
        #[serde(flatten)]
        checkout: Option<Checkout>,
    }
}

impl Git {
    /// Returns the git repository.
    #[inline]
    pub fn repo(&self) -> &str {
        match self {
            Self::Repo(repo) |
            Self::Detailed { repo, .. } => repo
        }
    }

    /// WRites the TOML form of `self` to `f`.
    #[inline]
    pub fn write_toml(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Repo(repo) => write!(f, "git = \"{}\"", repo),
            Self::Detailed { repo, checkout } => {
                write!(f, r#"git = {{ repo = "{}""#, repo)?;
                if let Some(checkout) = checkout {
                    write!(f, r#", {} = "{}""#, checkout.kind(), checkout)?;
                }
                write!(f, " }}")
            },
        }
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

/// The git checkout to use.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Checkout {
    /// The specific git branch.
    Branch(String),
    /// A specific git tag.
    Tag(String),
    /// A specific git revision.
    Rev(String),
}

impl fmt::Display for Checkout {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for Checkout {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Checkout {
    /// Returns the checkout string.
    #[inline]
    pub fn as_str(&self) -> &str {
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

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Parsed {
        git: Git,
    }

    impl Parsed {
        fn parse(toml: &str) -> Self {
            toml::de::from_str::<Parsed>(toml).unwrap()
        }
    }

    #[test]
    fn toml_repo() {
        let parsed = Parsed::parse(r#"
            git = "https://github.com/oceanpkg/ocean.git"
        "#);
        let expected = Parsed {
            git: Git::Repo("https://github.com/oceanpkg/ocean.git".into()),
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn toml_detailed() {
        let parsed = Parsed::parse(r#"
            [git]
            repo = "https://github.com/oceanpkg/ocean.git"
            tag = "lib-v0.0.7"
        "#);
        let expected = Parsed {
            git: Git::Detailed {
                repo: "https://github.com/oceanpkg/ocean.git".into(),
                checkout: Some(Checkout::Tag("lib-v0.0.7".into())),
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn toml_detailed_table() {
        let parsed = Parsed::parse(r#"
            git = { repo = "https://github.com/oceanpkg/ocean.git", tag = "lib-v0.0.7" }
        "#);
        let expected = Parsed {
            git: Git::Detailed {
                repo: "https://github.com/oceanpkg/ocean.git".into(),
                checkout: Some(Checkout::Tag("lib-v0.0.7".into())),
            },
        };
        assert_eq!(parsed, expected);
    }
}
