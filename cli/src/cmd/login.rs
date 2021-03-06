use super::prelude::*;
use oceanpkg::{
    api,
    auth::credentials::{Credentials, Registry},
};
use std::fs;

pub const NAME: &str = "login";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Log into Ocean for shipping drops")
        .arg(
            Arg::with_name("username")
                .takes_value(true)
                .required(true)
                .help("The username with which to login"),
        )
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    let username = matches
        .value_of("username")
        .unwrap_or_else(|| unreachable!("Required argument"));

    let password = rpassword::prompt_password_stdout(&formatln!(
        "Enter password for \"{}\":",
        username
    ))?;

    let credentials = api::v1::Credentials::basic_auth(username, &password);
    let token = api::v1::request_login_token(&credentials)?;

    // Serialize token into TOML string.
    let credentials = Credentials {
        registry: Some(Registry { token }),
    };
    let toml = toml::to_string_pretty(&credentials)?;

    // Write credentials.
    let credentials_path = config.rt.credentials_path();
    if let Some(parent) = credentials_path.parent() {
        fs::DirBuilder::new().recursive(true).create(parent)?;
    }
    fs::write(&credentials_path, &toml)?;

    println!("You are now logged in!");
    Ok(())
}
