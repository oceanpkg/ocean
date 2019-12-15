use super::super::prelude::*;

pub const NAME: &str = "rev";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Prints the git revision used to build Ocean")
}

pub fn run(_state: &mut Config, _matches: &ArgMatches) -> crate::Result {
    if let Some(rev) = crate::GIT_REV {
        println!("{}", rev);
        Ok(())
    } else {
        Err(failure::err_msg("`ocean` was not built with `git` available"))
    }
}
