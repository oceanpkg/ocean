extern crate clap;
extern crate oceanpkg;

use oceanpkg::Config;

#[macro_use]
mod macros;

mod cmd;

type Result<T = ()> = failure::Fallible<T>;

const ABOUT: &str = "
Flexibly manages packages

See https://www.oceanpkg.org for more info.";

fn app() -> clap::App<'static, 'static> {
    clap::App::new("ocean")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(ABOUT)
        .settings(&[
            clap::AppSettings::SubcommandRequiredElseHelp,
        ])
        .global_settings(&[
            clap::AppSettings::ColoredHelp,
            clap::AppSettings::VersionlessSubcommands,
            clap::AppSettings::DeriveDisplayOrder,
        ])
        .set_term_width(80)
        .subcommands(cmd::all())
        .arg(clap::Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Outputs more debugging information")
            .global(true))
}

fn main() {
    let mut config = Config::create()
        .unwrap_or_else(|error| {
            exit_error!("error: {}", error)
        });

    let matches = app().get_matches();
    if let (cmd, Some(matches)) = matches.subcommand() {
        cmd::run(&mut config, cmd, matches)
            .unwrap_or_else(|error| {
                exit_error!("error: {}", error)
            });
    } else {
        // SubcommandRequiredElseHelp
    }
}
