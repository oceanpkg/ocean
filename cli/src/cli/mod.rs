use crate::cmd;
use oceanpkg::Config;
use std::ffi::OsStr;

mod exec;

pub fn main(config: &mut Config) -> crate::Result {
    let args = cli().get_matches_safe()?;
    let args = resolve_aliases(config, args)?;

    match args.subcommand() {
        (subcommand, Some(args)) => {
            if let Some(run) = cmd::builtin_run_fn(subcommand) {
                run(config, args)
            } else {
                exec::run_external(config, subcommand, args)
            }
        }
        _ => Err(failure::err_msg("No subcommand provided")),
    }
}

fn cli() -> clap::App<'static, 'static> {
    clap::App::new("ocean")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(crate::ABOUT)
        .settings(&[clap::AppSettings::SubcommandRequiredElseHelp])
        .global_settings(&[
            clap::AppSettings::ColoredHelp,
            clap::AppSettings::VersionlessSubcommands,
            clap::AppSettings::DeriveDisplayOrder,
            clap::AppSettings::AllowExternalSubcommands,
        ])
        .set_term_width(80)
        .subcommands(cmd::all())
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Outputs more debugging information")
                .global(true),
        )
}

fn alias_cli() -> clap::App<'static, 'static> {
    cli()
        // We're not passing in the binary name, so skip parsing it.
        .setting(clap::AppSettings::NoBinaryName)
        // The `NoBinaryName` setting removes the name from errors.
        .bin_name("ocean")
}

/// Expands any built-in or user-defined aliases used in `args`.
fn resolve_aliases<'a>(
    config: &mut Config,
    mut args: clap::ArgMatches<'a>,
) -> crate::Result<clap::ArgMatches<'a>> {
    // Detect cycles by checking if we've seen the alias before. A pointer to
    // the alias start should work since we can get the start address from the
    // first reference returned by `UserConfig::parse_alias_os`.
    let mut seen_user_aliases = Vec::<*const u8>::new();

    while let (cmd, Some(sub_args)) = args.subcommand() {
        if cmd::builtin_run_fn(cmd).is_some() {
            // Built-in commands override user-defined aliases.

            if config.user.aliases.contains_key(cmd) {
                eprintln!(
                    "Ignoring user-defined alias `{}` because it is shadowed \
                     by a built-in command",
                    cmd
                );
            }

            // No need to continue resolving.
            break;
        } else if let Some(mut alias) = config.user.parse_alias_os(cmd) {
            // User-defined aliases override built-in aliases.

            // The empty string returns all values.
            if let Some(values) = sub_args.values_of_os("") {
                alias.extend(values);
            }

            // Perform alias cycle detection by checking the start address.
            if let Some(start) = alias.first() {
                let start = *start as *const OsStr as *const u8;
                if seen_user_aliases.contains(&start) {
                    return Err(failure::format_err!(
                        "User-defined alias `{}` produces an infinite cycle",
                        cmd
                    ));
                } else {
                    seen_user_aliases.push(start);
                }
            }

            args = alias_cli().get_matches_from_safe(alias)?;

            // The user's alias may resolve to another alias, so we need to
            // continue looping until all aliases are resolved.
            continue;
        } else if let Some(alias) = cmd::builtin_alias(cmd) {
            let mut alias = vec![OsStr::new(alias)];

            // The empty string returns all values.
            if let Some(values) = sub_args.values_of_os("") {
                alias.extend(values);
            }

            args = alias_cli().get_matches_from_safe(alias)?;

            // Built-in aliases always resolve directly to a real command, so
            // there's no need to continue looping.
            break;
        } else {
            // The command is neither an alias nor a built-in function.
            break;
        }
    }

    Ok(args)
}
