use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};
use reqwest::{
    Client,
    cookie::Cookie,
};
use super::prelude::*;

pub const NAME: &str = "login";

const API_TOKEN_FILE: &str = ".ocean_api_token";

#[derive(Serialize)]
struct Credentials<'a> {
    username: &'a str,
    password: &'a str,
}

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Log into Ocean for publishing")
        .arg(Arg::with_name("username")
            .takes_value(true)
            .required(true)
            .help("The username with which to login"))
}

pub fn run(matches: &ArgMatches) -> crate::Result {
    let username = matches.value_of("username")
        .unwrap_or_else(|| unreachable!("Required argument"));

    let api_url = crate::api_url().unwrap();
    let url = api_url.join("login").unwrap();

    println!("Logging into \"{}\"...", url);

    println!("Password for \"{}\":", username);

    let mut password = String::with_capacity(8);
    io::stdin().read_line(&mut password).unwrap();
    password.pop(); // Remove newline

    let client = Client::new();
    let response = client.post(url.as_str())
        .json(&Credentials { username, password: &password })
        .send()
        .unwrap();

    println!("Received headers: {:#?}", response.headers());

    let status = response.status();
    if !status.is_success() {
        exit_error!("Received response: \"{}\"", status);
    }

    let mut cookies = response.cookies();
    let api_cookie: Cookie = loop {
        if let Some(cookie) = cookies.next() {
            if cookie.name() == "token" {
                break cookie;
            }
        } else {
            exit_error!("Did not receive an API login token; aborting...");
        }
    };
    let api_token = api_cookie.value();
    println!("Received token \"{}\"...", api_token);

    Ok(())
}

fn ocean_api_token_path() -> Option<PathBuf> {
    let mut path = dirs::home_dir()?;
    path.push(".ocean_api_token");
    Some(path)
}

fn read_api_token(dir: &Path) -> crate::Result<String> {
    let token_path = ocean_api_token_path()
        .ok_or(failure::err_msg("Could not get user home"))?;

    let mut token = String::with_capacity(165);
    File::open(token_path)?
        .read_to_string(&mut token)?;

    Ok(token)
}
