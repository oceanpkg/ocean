use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    mem,
};
use oceanpkg::{
    api,
    drop::{
        Manifest,
        name::{Name, Query},
    },
};
use super::prelude::*;

pub const NAME: &str = "install";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .visible_alias("i")
        .about("Downloads and installs a drop")
        .arg(Arg::user_flag()
            .help("Drop(s) will be available to a specific user"))
        .arg(Arg::global_flag()
            .help("Drop(s) will be globally available to all users"))
        .arg(Arg::with_name("drop")
            .help("The package(s) to install")
            .multiple(true)
            .required(true))
        .arg(Arg::with_name("with")
            .help("Include optional dependencies")
            .long("with")
            .takes_value(true)
            .value_name("dep")
            .multiple(true))
        .arg(Arg::with_name("without")
            .help("Exclude recommended dependencies")
            .long("without")
            .takes_value(true)
            .value_name("dep")
            .multiple(true))
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    let with_deps: Vec<&Name> = matches
        .values_of("with")
        .map(name_values)
        .unwrap_or_default();

    let without_deps: Vec<&Name> = matches
        .values_of("without")
        .map(name_values)
        .unwrap_or_default();

    handle_conflicts(&with_deps, &without_deps);

    let install_target = matches.install_target();
    println!("Installing for {:?}", install_target);

    struct VersionedQuery<S> {
        scope: Option<S>,
        name: S,
        version: S,
    }

    impl<'a> VersionedQuery<&'a str> {
        fn file_name(&self) -> String {
            format!("{}@{}", self.name, self.version)
        }
    }

    let mut successes = Vec::<VersionedQuery<&str>>::new();

    if let Some(drops) = matches.values_of("drop") {
        for drop in drops {
            // TODO: Parse name
            println!("Installing \"{}\"...", drop);

            macro_rules! fail {
                ($error:expr) => { {
                    error!("failed to install \"{}\": {} (line {})", drop, $error, line!());
                    continue;
                } };
            }

            let query = Query::<&str>::parse_liberal(drop);
            let download = match download(config, query) {
                Ok(dl) => dl,
                Err(error) => fail!(error),
            };

            let drops_dir = config.rt.drops_dir(&install_target);
            let unpack_dir = {
                let mut dir = drops_dir.into_owned();
                dir.push(query.scope.unwrap_or("core"));
                dir
            };

            let manifest_path = match unpack(&download.path, &unpack_dir) {
                Ok(path) => unpack_dir.join(path),
                Err(error) => fail!(error),
            };

            let manifest = match Manifest::read_toml_file(&manifest_path) {
                Ok(m) => m,
                Err(error) => fail!(error),
            };

            let query = VersionedQuery {
                scope: query.scope,
                name: query.name,
                version: match query.version {
                    Some(v) => v,
                    None => {
                        fn leak(string: String) -> &'static str {
                            let slice = string.as_str() as *const str;
                            mem::forget(string);
                            unsafe { &*slice }
                        }
                        leak(manifest.meta.version.to_string())
                    },
                },
            };

            let drop_path = {
                let mut path = unpack_dir;
                path.push(query.file_name());
                path
            };

            let exe_path = drop_path.join(manifest.meta.exe_path());
            if !exe_path.is_file() {
                fail!(failure::format_err!(
                    "file not found: \"{}\"",
                    exe_path.display(),
                ));
            }

            let chmod_status = Command::new("chmod")
                .arg("+x")
                .arg(&exe_path)
                .status();
            match chmod_status {
                Ok(status) => if !status.success() {
                    fail!(failure::format_err!(
                        "failed to make \"{}\" executable",
                        exe_path.display(),
                    ));
                },
                Err(error) => fail!(error),
            }

            successes.push(query);
        }
    }

    // Get duration immediately after shipping finishes.
    let elapsed = config.rt.time_elapsed();

    if !successes.is_empty() {
        println!();
        println!("Successfully installed:");
        for query in successes {
            print!("- ");
            if let Some(scope) = query.scope {
                print!("{}/", scope);
            }
            print!("{} ({})", query.name, query.version);
            println!();
        }
    }

    println!();
    println!("Finished in {:?}", elapsed);
    Ok(())
}

/// Converts `values` to a vector of `Name`s if they're all valid, or exits with
/// an error code if any are not.
fn name_values(values: clap::Values) -> Vec<&Name> {
    values.map(Name::new)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| exit_error!(err))
}

/// Checks if `with` and `without` have any shared names, exiting with an error
/// code if they do.
fn handle_conflicts(with: &[&Name], without: &[&Name]) {
    let mut conflicts = without
        .iter()
        .filter(|name| with.contains(name));

    if let Some(first) = conflicts.next() {
        eprint!("Cannot be `--with` and `--without`: {}", first);
        for conflict in conflicts {
            eprint!(", {}", conflict);
        }
        eprintln!();
        std::process::exit(1);
    }
}

struct Download {
    #[allow(unused)]
    file: File,
    path: PathBuf,
}

fn download(config: &Config, drop: Query<&str>) -> crate::Result<Download> {
    let tarball_path = config.rt.tarball_cache_path(drop);

    if let Some(parent) = tarball_path.parent() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(parent)?;
    }

    let mut tarball_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&tarball_path)?;

    {
        let mut buf = BufWriter::new(&mut tarball_file);
        api::v1::download_drop(drop, &mut buf)?;
        buf.flush()?;
    }

    Ok(Download {
        file: tarball_file,
        path: tarball_path,
    })
}

// FIXME: Non-command unpacking fails.
// fn unpack(tarball: &mut File, destination: &Path) -> crate::Result {
//     oceanpkg::archive::unpack_tarball(tarball, destination)?;
//     Ok(())
// }

fn unpack(tarball: &Path, dir: &Path) -> crate::Result<String> {
    use std::process::{Command, Stdio};

    assert!(tarball.exists(), "{:?} does not exist", tarball);

    fs::DirBuilder::new()
        .recursive(true)
        .create(dir)?;

    // Runs `gunzip -c $download_path | tar xopf -`
    let mut gunzip = Command::new("gunzip")
        .arg("-c")
        .arg(tarball)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .spawn()?;
    let gunzip_stdout = gunzip.stdout.take()
        .ok_or(failure::err_msg("Could not get stdout handle for `gunzip`"))?;
    let tar_status = Command::new("tar")
        .args(&["xopf", "-"])
        .current_dir(dir)
        .stdin(gunzip_stdout)
        .status()?;

    failure::ensure!(tar_status.success(), "Failed to unpack download");

    // Runs `gunzip -c $download_path | tar -t */Ocean.toml -`
    let mut gunzip = Command::new("gunzip")
        .arg("-c")
        .arg(tarball)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .spawn()?;
    let gunzip_stdout = gunzip.stdout.take()
        .ok_or(failure::err_msg("Could not get stdout handle for `gunzip`"))?;
    let tar_output = Command::new("tar")
        .args(&["-t", "*/Ocean.toml", "-"])
        .current_dir(dir)
        .stdin(gunzip_stdout)
        .output()?;

    let mut manifest_path = String::from_utf8(tar_output.stdout)?;
    while manifest_path.ends_with(|c| c == '\n' || c == '\r') {
        manifest_path.pop();
    }

    Ok(manifest_path)
}
