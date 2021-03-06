use super::prelude::*;
use clap::ArgMatches;

pub const NAME: &str = "search";

pub fn cmd() -> App {
    SubCommand::with_name(NAME).about("Search for drop(s)").arg(
        Arg::with_name("drop")
            .help("The package name(s) to search for")
            .multiple(true)
            .required(true),
    )
}

pub fn run(_state: &mut Config, _matches: &ArgMatches) -> crate::Result {
    unimplemented!()
}
