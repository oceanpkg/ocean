#![allow(unused_imports)]

use super::*;
use crate::drop::{
    license::{self, SpdxLicense},
    name::{Name, Query},
    source::git::{self, Git, OCEAN_REPO},
    version::SemVer,
};

#[cfg(feature = "toml")]
fn manifests() -> Vec<(String, Manifest)> {
    let version = "0.1.0";
    let semver = SemVer::parse(version).unwrap();
    let repo = OCEAN_REPO;
    let home = "https://www.oceanpkg.org";
    let docs = "https://docs.oceanpkg.org";
    let wget = Query::<&str>::parse_liberal("wget");
    let meta = Meta {
        name: "ocean".to_owned(),
        display_name: Some("Ocean".to_owned()),
        description: "Cross-platform package manager".to_owned(),
        exe_path: None,
        version: semver.into(),
        conflicts: None,
        license: Some(SpdxLicense::Agpl3Only.id().to_owned()),
        authors: Some(vec![
            "Nikolai Vazquez".to_owned(),
            "Alex Farra".to_owned(),
            "Nicole Zhao".to_owned(),
        ]),
        readme: Some("README.md".to_owned()),
        changelog: Some("CHANGELOG.md".to_owned()),
        git: Some(Git {
            repo: repo.to_owned(),
            reference: Some(git::Ref::Tag(version.to_owned())),
        }),
        homepage: Some(home.to_owned()),
        documentation: Some(docs.to_owned()),
    };
    let header = format!(
        r#"
            [meta]
            name = "ocean"
            display-name = "Ocean"
            description = "Cross-platform package manager"
            version = "{version}"
            license = "AGPL-3.0"
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
            wget.to_owned(),
            DepInfo {
                version: "*".to_owned(),
                optional: false,
                git: Some(Git::new(
                    "https://git.savannah.gnu.org/git/wget.git",
                    git::Ref::branch("1.0"),
                )),
            },
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
                    (wget.to_owned(), "*".to_owned().into())
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

fn example_manifest() -> Manifest {
    Manifest {
        meta: Meta {
            name: "wumbo".to_owned(),
            display_name: Some("Wumbo".to_owned()),
            description: "Something silly".to_owned(),
            exe_path: Some("wumbo".to_owned()),
            version: SemVer::new(0, 1, 0).into(),
            conflicts: None,
            license: Some("MIT OR AGPL-3.0".to_owned()),
            authors: Some(vec![
                "Nikolai Vazquez".to_owned(),
                "Patrick Star".to_owned(),
            ]),
            readme: Some("../README.md".to_owned()),
            changelog: Some("../CHANGELOG.md".to_owned()),
            git: Some(Git::new(
                OCEAN_REPO,
                git::Ref::tag("v0.1.0"),
            )),
            homepage: Some("https://example.com".to_owned()),
            documentation: Some("https://example.com/docs".to_owned()),
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

mod json {
    use super::*;

    #[test]
    fn serialize_manifest() {
        let manifest = example_manifest();
        manifest.to_json(false).unwrap();
        manifest.to_json(true).unwrap();
    }
}
