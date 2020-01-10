use oceanpkg::system::open;
use super::prelude::*;

pub const NAME: &str = "docs";

const OCEAN_DOCS: &str = "https://docs.oceanpkg.org/";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Opens documentation in a browser")
        .arg(Arg::with_name("drops")
            .help("The drops to open documentation for")
            .multiple(true))
        .arg(Arg::with_name("print")
            .short("p")
            .long("print")
            .help("Simply print the documentation URL"))
}

pub fn run(_state: &mut Config, matches: &ArgMatches) -> crate::Result {
    let print_docs = matches.is_present("print");

    if let Some(drops) = matches.values_of("drops") {
        for drop in drops {
            unimplemented!("TODO: Open the documentation page for {:?}", drop);
        }
    } else if print_docs {
        println!("{}", OCEAN_DOCS);
    } else {
        open(&[OCEAN_DOCS])?;
    }
    Ok(())
}
