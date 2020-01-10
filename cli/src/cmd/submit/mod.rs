use super::prelude::*;
use oceanpkg::system::open;

mod bug;
mod feature;

pub const NAME: &str = "submit";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Creates a pre-populated GitHub issue")
        .arg(
            Arg::with_name("kind")
                .help("The type of issue: 'bug' or 'feature'")
                .possible_values(&["bug", "feature"])
                .hide_possible_values(true) // Format this ourselves.
                .required(true),
        )
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    let url = match matches.value_of("kind") {
        Some("bug") => bug::url(config),
        Some("feature") => feature::url(config),
        _ => unreachable!(), // Required argument
    };
    open(&[url])?;
    Ok(())
}
