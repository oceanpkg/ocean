#![allow(unused_imports)]

use super::*;
use crate::drop::{
    license::{self, SpdxLicense},
    name::{Name, QueryName},
    source::git::{self, Git, OCEAN_REPO},
    version::SemVer,
};

#[cfg(feature = "toml")]
fn manifests<'a>() -> Vec<(String, Manifest<'a>)> {
    let version = "0.1.0";
    let semver = SemVer::parse(version).unwrap();
    let repo = OCEAN_REPO;
    let home = "https://www.oceanpkg.org";
    let docs = "https://docs.oceanpkg.org";
    let wget = QueryName::parse("wget").unwrap();
    let meta = Meta {
        name: Name::OCEAN,
        description: "Cross-platform package manager",
        version: semver.into(),
        conflicts: None,
        license: Some(SpdxLicense::Apache2.into()),
        authors: Some(vec!["Nikolai Vazquez", "Alex Farra", "Nicole Zhao"]),
        readme: Some("README.md"),
        changelog: Some("CHANGELOG.md"),
        git: Some(Git {
            repo,
            reference: Some(git::Ref::Tag(version)),
        }),
        homepage: Some(home),
        documentation: Some(docs),
    };
    let header = format!(
        r#"
            [meta]
            name = "ocean"
            description = "Cross-platform package manager"
            version = "{version}"
            license = "Apache-2.0"
            authors = ["Nikolai Vazquez", "Alex Farra", "Nicole Zhao"]
            readme = "README.md"
            changelog = "CHANGELOG.md"
            git = {{ repo = "{repo}", tag = "{version}" }}
            homepage = "{homepage}"
            documentation = "{documentation}"
        "#,
        version = version,
        repo = repo,
        homepage = home,
        documentation = docs,
    );
    let detailed_deps: Deps = vec![
        (
            wget,
            DepInfo {
                version: "*",
                git: Some(Git {
                    repo: "https://git.savannah.gnu.org/git/wget.git",
                    reference: Some(git::Ref::Branch("1.0")),
                }.into()),
                optional: false,
            }.into(),
        )
    ].into_iter().collect();
    vec![
        (
            format!(
                r#"
                    {}
                    [dependencies]
                    wget = "*"
                "#,
                header,
            ),
            Manifest {
                meta: meta.clone(),
                deps: Some(vec![
                    (wget, "*".into())
                ].into_iter().collect()),
            }
        ),
        (
            format!(
                r#"
                    {}
                    [dependencies]
                    wget = {{ version = "*", git = {{ repo = "https://git.savannah.gnu.org/git/wget.git", branch = "1.0" }} }}
                "#,
                header,
            ),
            Manifest {
                meta: meta.clone(),
                deps: Some(detailed_deps.clone()),
            }
        ),
        (
            format!(
                r#"
                    {}
                    [dependencies.wget]
                    version = "*"
                    git = {{ repo = "https://git.savannah.gnu.org/git/wget.git", branch = "1.0" }}
                "#,
                header,
            ),
            Manifest { meta, deps: Some(detailed_deps) }
        ),
    ]
}

#[cfg(any(feature = "toml", feature = "serde_json"))]
fn example_manifest() -> Manifest<'static> {
    Manifest {
        meta: Meta {
            name: Name::new("wumbo").unwrap(),
            description: "Something silly",
            version: SemVer::new(0, 1, 0).into(),
            conflicts: None,
            license: Some(license::Expr::parse("MIT OR Apache-2.0").unwrap()),
            authors: Some(vec!["Nikolai Vazquez", "Patrick Star"]),
            readme: Some("../README.md"),
            changelog: Some("../CHANGELOG.md"),
            git: Some(Git {
                repo: OCEAN_REPO,
                reference: Some(git::Ref::Tag("v0.1.0")),
            }),
            homepage: Some("https://example.com"),
            documentation: Some("https://example.com/docs"),
        },
        deps: Some(vec![

        ].into_iter().collect()),
    }
}

#[cfg(feature = "toml")]
mod toml {
    use super::*;

    #[test]
    fn deserialize_manfiest() {
        for (toml, manifest) in manifests() {
            let parsed = Manifest::parse_toml(&toml).unwrap();
            assert_eq!(manifest, parsed, "\n{:#?}\n{:#?}\n", manifest, parsed);
        }
    }

    #[test]
    fn serialize_manifest() {
        let manifest = example_manifest();
        manifest.to_toml(false).unwrap();
        manifest.to_toml(true).unwrap();
    }
}

#[cfg(feature = "serde_json")]
mod json {
    use super::*;

    #[test]
    fn serialize_manifest() {
        let manifest = example_manifest();
        manifest.to_json(false).unwrap();
        manifest.to_json(true).unwrap();
    }
}
