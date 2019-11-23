//! Git repository information.

use std::fmt;

/// Information about a git repository where a drop or dependency can be found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    /// WRites the TOML form of `self` to `f`.
    #[inline]
    pub fn write_toml(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Git::Repo(repo) => write!(f, "git = \"{}\"", repo),
            Git::Detailed { repo, checkout } => {
                write!(f, r#"git = {{ repo = "{}""#, repo)?;
                if let Some(checkout) = checkout {
                    write!(f, r#", {} = "{}""#, checkout.variant_name(), checkout.as_str())?;
                }
                write!(f, " }}")
            },
        }
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    /// Returns the name of the variant: `branch`, `tag`, or `rev`.
    #[inline]
    pub fn variant_name(&self) -> &'static str {
        match self {
            Checkout::Branch(_) => "branch",
            Checkout::Tag(_) => "tag",
            Checkout::Rev(_) => "rev",
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
            tag = "lib-v0.0.6"
        "#);
        let expected = Parsed {
            git: Git::Detailed {
                repo: "https://github.com/oceanpkg/ocean.git",
                checkout: Some(Checkout::Tag("lib-v0.0.6")),
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn toml_detailed_table() {
        let parsed = Parsed::parse(r#"
            git = { repo = "https://github.com/oceanpkg/ocean.git", tag = "lib-v0.0.6" }
        "#);
        let expected = Parsed {
            git: Git::Detailed {
                repo: "https://github.com/oceanpkg/ocean.git",
                checkout: Some(Checkout::Tag("lib-v0.0.6")),
            },
        };
        assert_eq!(parsed, expected);
    }
}
