use super::prelude::*;
use oceanpkg::auth::Credentials;
use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub const NAME: &str = "ship";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Package and upload this drop to the registry")
        .arg(
            Arg::with_name("token")
                .help("Token to use when uploading")
                .long("token")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("manifest")
                .help("Path to Ocean.toml")
                .long("manifest")
                .takes_value(true),
        )
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    let package = oceanpkg::drop::Package::create(
        config.rt.current_dir(),
        matches.value_of_os("manifest"),
        None::<&Path>,
    )?;

    let credentials: String;
    let token = match matches.value_of("token") {
        Some(token) => token,
        None => {
            let credentials_path = config.rt.credentials_path();

            let mut credentials_file = match File::open(credentials_path) {
                Ok(file) => file,
                Err(error) => {
                    use io::ErrorKind;
                    match error.kind() {
                        ErrorKind::NotFound => {
                            failure::bail!("please run `ocean login` first")
                        }
                        _ => return Err(error.into()),
                    }
                }
            };

            let mut credentials_buf = String::with_capacity(256);
            credentials_file.read_to_string(&mut credentials_buf)?;
            credentials = credentials_buf;

            let credentials: Credentials<&str> = toml::from_str(&credentials)?;
            credentials
                .registry
                .ok_or_else(|| {
                    failure::err_msg("please run `ocean login` first")
                })?
                .token
        }
    };

    oceanpkg::api::v1::ship(&package, token)?;

    // Get duration immediately after shipping finishes.
    let elapsed = config.rt.time_elapsed();

    println!("Successfully shipped \"{}\"!", package.manifest.meta.name);
    println!("Finished in {:?}", elapsed);

    Ok(())
}
