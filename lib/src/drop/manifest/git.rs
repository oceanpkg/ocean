//! Git repository information.

use std::fmt;
use super::{Detailed};

/// Information about a git repository where a drop or dependency can be found.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Git<'a> {
    /// Where the git repository is located.
    #[serde(alias = "repository")]
    pub repo: &'a str,
    /// The specific branch to use.
    #[serde(flatten)]
    pub checkout: Option<Checkout<'a>>,
}

impl<'a> Detailed for Git<'a> {
    type Simple = &'a str;
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
    use super::{
        *,
        super::Flexible,
    };

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct Parsed<'a> {
        #[serde(borrow)]
        git: Flexible<Git<'a>>,
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
        let git = Flexible::<Git>::Simple("https://github.com/oceanpkg/ocean.git");
        let expected = Parsed { git };
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
            git: Git {
                repo: "https://github.com/oceanpkg/ocean.git",
                checkout: Some(Checkout::Tag("lib-v0.0.7")),
            }.into(),
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn toml_detailed_table() {
        let parsed = Parsed::parse(r#"
            git = { repo = "https://github.com/oceanpkg/ocean.git", tag = "lib-v0.0.7" }
        "#);
        let expected = Parsed {
            git: Git {
                repo: "https://github.com/oceanpkg/ocean.git",
                checkout: Some(Checkout::Tag("lib-v0.0.7")),
            }.into(),
        };
        assert_eq!(parsed, expected);
    }
}
