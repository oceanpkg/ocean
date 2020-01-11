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

/// A function suitable for running a subcommand with its own `ArgMatches`.
pub type RunFn = fn(&mut Config, &ArgMatches) -> crate::Result;

/// Returns the `run` function pointer for `subcommand`.
pub fn builtin_run_fn(subcommand: &str) -> Option<RunFn> {
    #[rustfmt::skip]
    let run = match subcommand {
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
        _               => return None,
    };
    Some(run)
}

/// Returns a pre-defined aliased command.
pub fn builtin_alias(alias: &str) -> Option<&'static str> {
    // Default aliases should be for commands of 4 characters or more.
    //
    // Destructive commands like `uninstall` should never have a built-in alias.
    #[rustfmt::skip]
    let alias = match alias {
        "i"  => install::NAME,
        "ls" => list::NAME,
        "s"  => search::NAME,
        _    => return None,
    };
    Some(alias)
}
