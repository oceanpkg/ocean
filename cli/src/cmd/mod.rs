pub mod prelude;
use self::prelude::{App, ArgMatches, State};

/// Handles the creation of modules for each subcommand as well as running the
/// appropriate subcommand for a pair of name string and matches.
macro_rules! cmds {
    ($(
        $(#[$meta:meta])*
        $cmd:ident,
    )+) => {
        $(
            $(#[$meta])*
            pub mod $cmd;
        )+

        /// Returns all of Ocean's subcommands to pass into `App::subcommands`.
        pub fn all() -> Vec<App> {
            vec![$($cmd::cmd(),)+]
        }

        /// Runs the command for `name` with its associated `ArgMatches`.
        pub fn run(state: &mut State, name: &str, matches: &ArgMatches) -> crate::Result {
            let run = match name {
                $($cmd::NAME => $cmd::run,)+
                _ => unreachable!("could not match command {:?}", name),
            };
            run(state, matches)
        }
    };
}

cmds! {
    list,
    new,
    search,
    install,
    uninstall,
    update,
    run,
    cfg,

    #[path = "self/mod.rs"]
    self_, // `self` is a keyword

    login,

    home,
    docs,
    source,
}
