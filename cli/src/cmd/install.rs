use oceanpkg::drop::Name;
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

pub fn run(matches: &ArgMatches) {
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

    if let Some(values) = matches.values_of_os("drop") {
        for value in values {
            println!("Installing {:?}...", value);
            // TODO: Parse name
            // TODO: Install package
        }
    }
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
