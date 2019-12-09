use super::prelude::*;

pub const NAME: &str = "uninstall";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        // TODO: Should we have shorthand aliases for uninstall?
        // .visible_aliases(&["un", "remove", "rm"])
        .about("Removes a drop")
        .arg(Arg::global_flag()
            .help("Remove the drop for all users"))
        .arg(Arg::with_name("drop")
            .help("The package(s) to remove")
            .multiple(true)
            .required(true))
}

pub fn run(_state: &mut State, matches: &ArgMatches) -> crate::Result {
    let install_target = matches.install_target();
    println!("Uninstalling for {:?}", install_target);

    if let Some(values) = matches.values_of_os("drop") {
        for value in values {
            println!("Uninstalling {:?}...", value);
        }
    }

    unimplemented!()
}
