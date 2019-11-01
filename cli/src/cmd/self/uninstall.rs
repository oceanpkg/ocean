use clap::ArgMatches;
use super::super::prelude::*;

pub const NAME: &str = "uninstall";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Uninstall Ocean")
}

pub fn run(_matches: &ArgMatches) {
    unimplemented!()
}
