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
        version: Flexible::Simple(semver),
        conflicts: None,
        license: Some(SpdxLicense::Apache2.into()),
        authors: Some(vec!["Nikolai Vazquez", "Alex Farra", "Nicole Zhao"]),
        readme: Some("README.md"),
        changelog: Some("CHANGELOG.md"),
        git: Some(Git {
            repo,
            checkout: Some(git::Checkout::Tag(version)),
        }.into()),
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
                    (wget, Flexible::Simple("*"))
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
    fn test<'t, 'm: 't>(toml: &'t str, manifest: &Manifest<'m>) {
        let parsed = Manifest::<'t>::parse_toml(&toml).unwrap();

        // FIXME: Remove need to make lifetimes match to appease the borrow
        // checker. This is required because introducing `Flexible` causes the
        // the borrow checker to think that 't and 'm are invariant, complaining
        // that 't does not live as long as 'm. This behavior of the `PartialEq`
        // impl for `Manifest` seems to be a compiler bug.
        //
        // Note that changing it to 't: 'm results in the compiler complaining
        // that `toml` at the call site in the loop does not live long enough.
        let manifest: &Manifest<'t> = unsafe {
            std::mem::transmute(manifest)
        };

        assert_eq!(*manifest, parsed, "\n{} != {}", manifest, parsed);
    }
    for (toml, manifest) in manifests() {
        test(&*toml, &manifest);
    }
}
