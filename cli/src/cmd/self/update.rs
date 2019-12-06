use clap::ArgMatches;
use super::super::prelude::*;

pub const NAME: &str = "update";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Download and install updates to Ocean")
}

pub fn run(_matches: &ArgMatches) -> crate::Result {
    unimplemented!()
}
