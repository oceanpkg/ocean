use super::*;
use crate::drop::{
    license::SpdxLicense,
    name::{Name, QueryName},
    version::SemVer,
};

fn manifests<'a>() -> Vec<(String, Manifest<'a>)> {
    let version = "0.1.0";
    let semver = SemVer::parse(version).unwrap();
    let repo = env!("CARGO_PKG_REPOSITORY");
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
            checkout: Some(git::Checkout::Tag(version)),
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
                    checkout: Some(git::Checkout::Branch("1.0")),
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

#[test]
fn parse_manfiest() {
    for (toml, manifest) in manifests() {
        let parsed = Manifest::parse_toml(&toml).unwrap();
        assert_eq!(manifest, parsed, "\n{} != {}", manifest, parsed);
    }
}
