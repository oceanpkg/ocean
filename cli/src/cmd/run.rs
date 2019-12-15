use std::process::{
    Command,
    exit,
};
use oceanpkg::{
    drop::{
        Manifest,
        name::Query,
    },
    install::InstallTarget,
};
use super::prelude::*;

pub const NAME: &str = "run";

const AFTER_HELP: &str = "\
All arguments following the two dashes (`--`) are passed directly to the drop \
as its own arguments. Any arguments designated for Ocean should go before the \
`--`.

Example:

    $ ocean run wget -- https://example.com
";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .about("Executes a drop")
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::TrailingVarArg,
        ])
        .arg(Arg::global_flag()
            .help("Execute the drop that's available to all users"))
        .arg(Arg::with_name("drop")
            .help("Name of the target drop to run")
            .required(true))
        .arg(Arg::with_name("args")
            .help("Arguments passed directly to the drop")
            .last(true)
            .multiple(true))
        .after_help(AFTER_HELP)
}

pub fn run(config: &mut Config, matches: &ArgMatches) -> crate::Result {
    if let Some(drop) = matches.value_of("drop") {
        let query = Query::<&str>::parse_liberal(drop);

        let scope = query.scope.unwrap_or("core");
        let name = query.name;
        let version = query.version.ok_or_else(|| {
            failure::err_msg("Provided query must include a version")
        })?;

        let query_string = format!("{}/{}@{}", scope, name, version);

        let drops_dir = config.rt.drops_dir(&InstallTarget::CurrentUser);
        let drop_path = drops_dir.join(&query_string);

        let manifest_path = drop_path.join(Manifest::FILE_NAME);
        if !manifest_path.exists() {
            failure::bail!("Could not run \"{}\"; please install it", query_string);
        }

        let manifest = Manifest::read_toml_file(&manifest_path)?;

        let exe_path = drop_path.join(manifest.meta.exe_path());

        let mut cmd = Command::new(&exe_path);
        if let Some(args) = matches.values_of("args") {
            cmd.args(args);
        }

        let status = cmd.status()?;
        let code = if status.success() {
            status.code().unwrap_or(0)
        } else {
            status.code().unwrap_or(1)
        };
        exit(code);
    } else {
        // ArgRequiredElseHelp
    }

    Ok(())
}
