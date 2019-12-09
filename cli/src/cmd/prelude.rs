use std::{
    cmp,
    convert::TryFrom,
    mem,
};
pub use clap::{AppSettings, ArgMatches, SubCommand};
pub use oceanpkg::install::InstallTarget;
pub use crate::State;

pub type App = clap::App<'static, 'static>;
pub type Arg = clap::Arg<'static, 'static>;

/// Extended functionality for `ArgMatches`.
pub trait ArgMatchesExt {
    /// Returns how drops get installed by checking whether a `"global"`
    /// argument was used.
    fn install_target(&self) -> InstallTarget;

    /// Returns the level of logging that should be used.
    fn log_level(&self) -> Option<log::LevelFilter>;
}

impl ArgMatchesExt for ArgMatches<'_> {
    fn install_target(&self) -> InstallTarget {
        if self.is_present("global") {
            InstallTarget::Global
        } else if let Some(user) = self.value_of("user") {
            InstallTarget::SpecificUser(user.to_owned())
        } else {
            InstallTarget::CurrentUser
        }
    }

    fn log_level(&self) -> Option<log::LevelFilter> {
        if self.is_present("silent") {
            Some(log::LevelFilter::Off)
        } else if self.is_present("verbose") {
            match self.value_of("verbose").map(str::parse::<usize>) {
                Some(Ok(level)) => {
                    let max_level = log::LevelFilter::max() as usize;
                    let min_level = cmp::min(level, max_level);
                    unsafe { Some(mem::transmute(min_level)) }
                },
                None => Some(log::LevelFilter::Error),
                Some(Err(_)) => None,
            }
        } else {
            None
        }
    }
}

pub trait ArgExt {
    /// The common `--all`/`-a` flag.
    fn all_flag() -> Self;

    /// The common `--global`/`-g` flag.
    fn global_flag() -> Self;

    /// The common `--user`/`-u` flag that takes a username value.
    fn user_flag() -> Self;
}

impl ArgExt for clap::Arg<'_, '_> {
    fn all_flag() -> Self {
        Arg::with_name("all")
            .short("a")
            .long("all")
    }

    fn global_flag() -> Self {
        Arg::with_name("global")
            .short("g")
            .long("global")
    }

    fn user_flag() -> Self {
        Arg::with_name("user")
            .short("u")
            .long("user")
            .takes_value(true)
            .number_of_values(1)
    }
}
