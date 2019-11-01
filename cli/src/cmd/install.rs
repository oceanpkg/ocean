use super::prelude::*;

pub const NAME: &str = "install";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .visible_alias("i")
        .about("Downloads and installs a drop")
        .arg(Arg::global_flag()
            .help("Drop(s) will be globally available to all users"))
        .arg(Arg::with_name("drop")
            .help("The package(s) to install")
            .multiple(true)
            .required(true))
}

pub fn run(matches: &ArgMatches) {
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
