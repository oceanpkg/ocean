use super::super::prelude::*;

pub const NAME: &str = "uninstall";

pub fn cmd() -> App {
    SubCommand::with_name(NAME).about("Uninstall Ocean")
}

pub fn run(_state: &mut Config, _matches: &ArgMatches) -> crate::Result {
    unimplemented!()
}
