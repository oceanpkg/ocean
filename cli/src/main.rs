extern crate clap;
extern crate oceanpkg;

#[macro_use]
mod macros;

mod cmd;
mod state;

#[doc(inline)]
pub use self::state::State;

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
            .global(true)
            .takes_value(true)
            .min_values(0)
            .max_values(1)
            .conflicts_with("silent"))
        .arg(clap::Arg::with_name("silent")
            .short("s")
            .long("silent")
            .help("Silence standard output and error"))
}

fn main() {
    let mut state = State::new()
        .unwrap_or_else(|error| {
            exit_error!("error: {}", error)
        });

    let matches = app().get_matches();
    if let (cmd, Some(matches)) = matches.subcommand() {
        cmd::run(&mut state, cmd, matches)
            .unwrap_or_else(|error| {
                exit_error!("error: {}", error)
            });
    } else {
        // SubcommandRequiredElseHelp
    }
}
