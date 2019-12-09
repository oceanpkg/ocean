pub mod prelude;
use self::prelude::{App, ArgMatches, ArgMatchesExt, State};

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
            if let Some(log_level) = matches.log_level() {
                panic!("Setting logging level to {}", log_level);
                state.log_level = log_level;
            }

            fern::Dispatch::new()
                .level(state.log_level)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][{}] {}",
                        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .apply()?;

            log::trace!("Began logging");

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
