use std::process::exit;
use super::prelude::*;
use oceanpkg::drop::kind::Exe;

pub const NAME: &str = "run";

const AFTER_HELP: &str = "\
All arguments following the two dashes (`--`) are passed directly to the drop \
as its own arguments. Any arguments designated for Ocean should go before the \
`--`.

Example:

    $ ocean run wget -- https://example.com
";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Executes a drop")
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::TrailingVarArg,
        ])
        .arg(Arg::global_flag()
            .help("Execute the drop that's available to all users"))
        .arg(Arg::with_name("drop")
            .help("Name of the target drop to run")
            .required(true))
        .arg(Arg::with_name("args")
            .help("Arguments passed directly to the drop")
            .last(true)
            .multiple(true))
        .after_help(AFTER_HELP)
}

pub fn run(matches: &ArgMatches) -> crate::Result {
    let install_target = matches.install_target();

    #[allow(unreachable_code)]
    match matches.value_of_os("drop") {
        Some(drop) => {
            #[allow(unused_variables)]
            let exe: Exe = unimplemented!("TODO: Get {:?} executable for {:?} if available", drop, install_target);
            match exe.command(&install_target) {
                Ok(mut cmd) => {
                    if let Some(args) = matches.values_of("args") {
                        cmd.args(args);
                    }
                    match cmd.status() {
                        Ok(status) => {
                            let code = if status.success() {
                                status.code().unwrap_or(0)
                            } else {
                                status.code().unwrap_or(1)
                            };
                            exit(code);
                        },
                        Err(err) => {
                            unimplemented!("TODO: Handle {:?} gracefully", err);
                        },
                    }
                },
                Err(err) => unimplemented!("TODO: Handle {:?} gracefully", err),
            }
        },
        None => unreachable!(),
    }
}
