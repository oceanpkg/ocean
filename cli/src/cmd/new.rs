use super::prelude::*;
use oceanpkg::drop::Name;
use std::path::Path;

pub const NAME: &str = "new";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Create a new, pre-filled drop manifest")
        .arg(
            Arg::with_name("name").takes_value(true).help(
                "The name of the drop; default is current directory name",
            ),
        )
        .arg(
            Arg::with_name("path")
                .takes_value(true)
                .long("path")
                .short("p")
                .help("The path where to write the manifest"),
        )
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    let path: &Path = match matches.value_of_os("path") {
        Some(path) => path.as_ref(),
        None => &config.rt.current_dir,
    };

    let name = path.file_name().unwrap_or("".as_ref());
    match Name::new(name) {
        Ok(name) => {
            unimplemented!("TODO: Spit out manifest for '{}'", name);
        }
        Err(error) => unimplemented!("TODO: Handle '{}'", error),
    }
}
