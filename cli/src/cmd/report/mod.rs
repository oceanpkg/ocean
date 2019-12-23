use oceanpkg::system::open;
use super::prelude::*;

mod bug;

// TODO: Consider renaming this to something flexible like `issue` or `submit`
// so then we can also have `ocean submit feature-request`.
pub const NAME: &str = "report";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Creates a pre-populated GitHub issue with configuration info")
        .arg(Arg::with_name("kind")
            .possible_value("bug")
            .required(true))
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    let url = match matches.value_of("kind") {
        Some("bug") => bug::url(config),
        _ => unreachable!(), // Required argument
    };
    open(&[url])?;
    Ok(())
}
