use super::prelude::*;
use oceanpkg::system::open;

pub const NAME: &str = "home";

const OCEAN_HOME: &str = "https://www.oceanpkg.org/";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Opens the homepage in a browser")
        .arg(
            Arg::with_name("drops")
                .help("The drops to open home pages for")
                .multiple(true),
        )
        .arg(
            Arg::with_name("print")
                .short("p")
                .long("print")
                .help("Simply print the homepage URL"),
        )
}

pub fn run(_state: &mut Config, matches: &ArgMatches) -> crate::Result {
    let print_home = matches.is_present("print");

    if let Some(drops) = matches.values_of("drops") {
        for drop in drops {
            unimplemented!("TODO: Open the homepage for {:?}", drop);
        }
    } else if print_home {
        println!("{}", OCEAN_HOME);
    } else {
        open(&[OCEAN_HOME])?;
    }
    Ok(())
}
