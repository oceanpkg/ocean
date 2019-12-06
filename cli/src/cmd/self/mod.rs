use super::prelude::*;

mod uninstall;
mod update;

pub const NAME: &str = "self";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Modify the Ocean installation")
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::DeriveDisplayOrder,
        ])
        .subcommands(vec![
            update::cmd(),
            uninstall::cmd(),
        ])
}

pub fn run(state: &mut crate::State, matches: &ArgMatches) -> crate::Result {
    if let (command, Some(matches)) = matches.subcommand() {
        let run = match command {
            update::NAME    => update::run,
            uninstall::NAME => uninstall::run,
            _ => unreachable!("could not match command {:?}", command),
        };
        run(state, matches)
    } else {
        // SubcommandRequiredElseHelp
        unreachable!()
    }
}
