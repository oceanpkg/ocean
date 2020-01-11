extern crate clap;
extern crate oceanpkg;

#[macro_use]
mod macros;

mod cli;
mod cmd;

type Result<T = ()> = failure::Fallible<T>;

/// The git revision for the version built.
pub const GIT_REV: Option<&str> = option_env!("OCEAN_GIT_REV");

const ABOUT: &str = "
Flexibly manages packages

See https://www.oceanpkg.org for more info.";

fn main() {
    let mut config = oceanpkg::Config::create()
        .unwrap_or_else(|error| exit_error!("error: {}", error));

    if let Err(error) = cli::main(&mut config) {
        exit_error!(error);
    }
}
