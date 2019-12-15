use super::prelude::*;

pub const NAME: &str = "list";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .visible_alias("ls")
        .about("List installed drops")
        .arg(Arg::user_flag()
            .help("List drops locally available to a specific user"))
        .arg(Arg::global_flag()
            .help("List drops globally available to all users"))
}

pub fn run(_state: &mut Config, matches: &ArgMatches) -> crate::Result {
    let install_target = matches.install_target();
    unimplemented!("TODO: List for {:?}", install_target);
}
