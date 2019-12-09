use oceanpkg::system::open;
use super::prelude::*;

pub const NAME: &str = "source";

const OCEAN_REPO: &str = "https://github.com/oceanpkg/ocean/";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Opens the source code repository in a browser")
        .arg(Arg::with_name("drops")
            .help("The drops to the repository for")
            .multiple(true))
        .arg(Arg::with_name("print")
            .short("p")
            .long("print")
            .help("Simply print the repository URL"))
}

pub fn run(_state: &mut State, matches: &ArgMatches) -> crate::Result {
    let print_repo = matches.is_present("print");

    if let Some(drops) = matches.values_of("drops") {
        for drop in drops {
            unimplemented!("TODO: Open the repository page for {:?}", drop);
        }
    } else {
        if print_repo {
            println!("{}", OCEAN_REPO);
        } else {
            open(&[OCEAN_REPO])?;
        }
    }
    Ok(())
}
