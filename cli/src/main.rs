extern crate clap;
extern crate oceanpkg;

#[macro_use]
mod macros;

mod cmd;
mod state;

use self::state::State;

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
    let matches = app().get_matches();
    if let (command, Some(matches)) = matches.subcommand() {
        let run = match command {
            cmd::list::NAME      => cmd::list::run,
            cmd::new::NAME       => cmd::new::run,
            cmd::search::NAME    => cmd::search::run,
            cmd::install::NAME   => cmd::install::run,
            cmd::uninstall::NAME => cmd::uninstall::run,
            cmd::update::NAME    => cmd::update::run,
            cmd::run::NAME       => cmd::run::run,
            cmd::cfg::NAME       => cmd::cfg::run,
            cmd::self_::NAME     => cmd::self_::run,
            _ => unreachable!("could not match command {:?}", command),
        };
        let mut state = State::new()
            .unwrap_or_else(|error| exit_error!(error));
        run(&mut state, matches)
            .unwrap_or_else(|error| exit_error!(error));
    } else {
        // SubcommandRequiredElseHelp
    }
}
