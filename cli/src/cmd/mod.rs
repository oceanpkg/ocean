pub mod prelude;
use self::prelude::App;

pub mod cfg;
pub mod install;
pub mod run;
pub mod list;
pub mod search;
pub mod uninstall;

#[path = "self/mod.rs"]
pub mod self_; // `self` is a keyword

/// Returns all of Ocean's subcommands to pass into `App::subcommands`.
pub fn all() -> Vec<App> {
    vec![
        list::cmd(),
        search::cmd(),
        install::cmd(),
        uninstall::cmd(),
        run::cmd(),
        cfg::cmd(),
        self_::cmd(),
    ]
}
