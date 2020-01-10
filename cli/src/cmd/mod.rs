pub mod prelude;
use prelude::{App, ArgMatches, Config};

mod config;
mod docs;
mod home;
mod install;
mod list;
mod login;
mod new;
mod package;
mod run;
mod search;
#[path = "self/mod.rs"]
mod self_; // `self` is a keyword
mod ship;
mod source;
mod submit;
mod uninstall;
mod update;

/// Returns all of Ocean's subcommands to pass into `App::subcommands`.
pub fn all() -> Vec<App> {
    vec![
        list::cmd(),
        new::cmd(),
        search::cmd(),
        install::cmd(),
        uninstall::cmd(),
        update::cmd(),
        run::cmd(),
        config::cmd(),
        self_::cmd(),
        login::cmd(),
        ship::cmd(),
        package::cmd(),
        home::cmd(),
        docs::cmd(),
        source::cmd(),
        submit::cmd(),
    ]
}

/// Runs the command for `name` with its associated `ArgMatches`.
pub fn run(
    config: &mut Config,
    name: &str,
    matches: &ArgMatches,
) -> crate::Result {
    #[rustfmt::skip]
    let run = match name {
        list::NAME      => list::run,
        new::NAME       => new::run,
        search::NAME    => search::run,
        install::NAME   => install::run,
        uninstall::NAME => uninstall::run,
        update::NAME    => update::run,
        run::NAME       => run::run,
        config::NAME    => config::run,
        self_::NAME     => self_::run,
        login::NAME     => login::run,
        ship::NAME      => ship::run,
        package::NAME   => package::run,
        home::NAME      => home::run,
        docs::NAME      => docs::run,
        source::NAME    => source::run,
        submit::NAME    => submit::run,
        _ => unreachable!("could not match command {:?}", name),
    };
    run(config, matches)
}
