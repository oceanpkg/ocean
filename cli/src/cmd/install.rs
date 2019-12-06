use std::{
    convert::TryFrom,
    fs::{DirBuilder, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use oceanpkg::drop::{
    Manifest,
    name::{Name, Query},
};
use url::Url;
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

pub fn run(matches: &ArgMatches) -> crate::Result {
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
    println!("Installing for {:?}...", install_target);

    if let Some(drops) = matches.values_of("drop") {
        for drop in drops {
            let drop = Query::<&str>::parse_liberal(drop);
            println!("Installing \"{}\"...", drop);

            if let Err(error) = install(&drop) {
                exit_error!(error);
            }
        }
    }

    unimplemented!()
}

/// Converts `values` to a vector of `Name`s if they're all valid, or exits with
/// an error code if any are not.
fn name_values<'a>(values: clap::Values<'a>) -> Vec<&'a Name> {
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

fn download_url(drop: &Query<&str>) -> Result<Url, url::ParseError> {
    drop.join_to_url(&crate::api_url()?)
}

fn download_dir(drop: &Query<&str>) -> crate::Result<PathBuf> {
    let mut path = dirs::download_dir()
        .ok_or(failure::err_msg("Cannot get download directory for user"))?;
    path.push("ocean");
    path.push(drop.scope.unwrap_or("core"));
    path.push(drop.name);
    path.push(drop.version.unwrap_or("latest"));
    Ok(path)
}

fn request(drop: &Query<&str>) -> crate::Result<reqwest::Response> {
    let url = download_url(drop)?;
    println!("Downloading from \"{}\"...", url);

    let client = reqwest::Client::new();
    let response = client.get(url.as_str()).send()?;
    println!("Received headers: {:#?}", response.headers());

    let status = response.status();
    ensure!(
        status.is_success(),
        "Received response: \"{}\"", status,
    );

    Ok(response)
}

fn download(drop: &Query<&str>, dir: &Path) -> crate::Result<PathBuf> {
    let path = dir.join("archive.tar.gz");
    println!("Downloading to \"{}\"...", dir.display());

    if path.is_file() {
        println!("Found existing archive! Skipping download...");
        return Ok(path);
    }

    let mut response = request(drop)?;
    DirBuilder::new().recursive(true).create(dir)?;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .create_new(true)
        .open(&path)?;

    let content_length = response.content_length().unwrap_or(0);
    let buf_len = usize::try_from(content_length)
        .unwrap_or(usize::max_value());

    let mut writer = io::BufWriter::with_capacity(buf_len, &mut file);
    let total_len = response.copy_to(&mut writer)?;
    writer.flush()?;
    println!("Downloaded {} bytes...", total_len);
    Ok(path)
}

fn unpack(archive: &Path, dir: &Path) -> crate::Result {
    // Runs `gunzip -c $download_path | tar xopf -`
    let mut gunzip = Command::new("gunzip")
        .arg("-c")
        .arg(archive)
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

    ensure!(tar_status.success(), "Failed to unpack download");

    Ok(())
}

fn install(drop: &Query<&str>) -> crate::Result {
    let download_dir  = download_dir(drop)?;
    let download_path = download(drop, &download_dir)?;
    unpack(&download_path, &download_dir)?;

    let manifest_path = download_dir.join("Ocean.toml");
    let manifest = Manifest::read_toml_file(&manifest_path).map_err(|error| {
        format_err!("Failed to read \"{}\": {}", manifest_path.display(), error)
    })?;

    println!("{:#?}", manifest);

    Ok(())
}
