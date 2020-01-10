use super::prelude::*;

pub const NAME: &str = "package";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Assemble the local package into a distributable tarball")
        .arg(
            Arg::with_name("manifest")
                .help("Path to Ocean.toml")
                .long("manifest")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("The directory where to output")
                .short("o")
                .long("output")
                .takes_value(true),
        )
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    let package = oceanpkg::drop::Package::create(
        config.rt.current_dir(),
        matches.value_of_os("manifest"),
        matches.value_of_os("output"),
    )?;
    // Get duration immediately after packaging finishes.
    let elapsed = config.rt.time_elapsed();

    let tarball = &package.path;
    let tarball = match tarball.strip_prefix(config.rt.current_dir()) {
        Ok(suffix) => suffix,
        Err(_) => &tarball,
    };

    println!("Successfully packaged \"{}\"!", tarball.display());
    println!("Finished in {:?}", elapsed);

    Ok(())
}
