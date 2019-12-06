use super::prelude::*;

pub const NAME: &str = "update";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Updates installed packages")
        .arg(Arg::all_flag()
            .help("Update all drops")
            .conflicts_with("drop"))
        .arg(Arg::global_flag()
            .help("Update the drop for all users"))
        .arg(Arg::user_flag()
            .help("Update the drop for a specific user"))
        .arg(Arg::with_name("drop")
            .help("The package(s) to update")
            .multiple(true)
            .required_unless("all"))
}

pub fn run(matches: &ArgMatches) -> crate::Result {
    let install_target = matches.install_target();
    println!("Updating for {:?}", install_target);

    if let Some(values) = matches.values_of_os("drop") {
        for value in values {
            println!("Updating {:?}...", value);
        }
    } else if matches.is_present("all") {
        println!("Updating all packages...")
    }

    unimplemented!()
}
