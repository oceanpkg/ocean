use std::{env, fs, io, process};
use oceanpkg::cfg::file::*;
use super::prelude::*;

pub const NAME: &str = "config";

pub fn cmd() -> App {
    SubCommand::with_name(NAME)
        .visible_alias("cfg")
        .about("Configure the settings used by Ocean")
        .arg(Arg::global_flag()
            .help("The global configuration file for all users"))
}

pub fn run(_state: &mut State, matches: &ArgMatches) -> crate::Result {
    let install_target = matches.install_target();

    let config_dir = match install_target.cfg_dir() {
        Ok(dir) => dir,
        Err(err) => {
            unimplemented!("{}", err);
        },
    };

    if !config_dir.exists() {
        if let Err(err) = fs::create_dir(&config_dir) {
            match err.kind() {
                io::ErrorKind::PermissionDenied => {
                    panic!("Permission to create {:?} denied", config_dir);
                },
                _ => panic!("{}", err),
            }
        }
    }

    if !config_dir.is_dir() {
        panic!("Found non-directory file at {:?}", config_dir);
    }

    let config_file = match CfgFile::find(&config_dir) {
        Ok(file) => file,
        Err(err) => match err.reason {
            NotFoundReason::Io(err) => unimplemented!("{}", err),
            NotFoundReason::NoMatch => {
                let path = config_dir.join("ocean.toml");
                let file = match fs::File::create(&path) {
                    Ok(file) => file,
                    Err(err) => match err.kind() {
                        io::ErrorKind::PermissionDenied => {
                            panic!("Permission to create {:?} denied", path);
                        },
                        _ => panic!("{}", err),
                    }
                };
                CfgFile { path, fmt: CfgFileFmt::Toml, handle: Some(file) }
            },
        },
    };

    println!("Found config file: {:#?}", config_file);

    #[cfg(unix)]
    let editor = if let Some(editor) = env::var_os("VISUAL") {
        editor
    } else if let Some(editor) = env::var_os("EDITOR") {
        editor
    } else {
        panic!("Could not determine editor");
    };

    #[cfg(windows)]
    let editor: ffi::OsString = unimplemented!();

    process::Command::new(editor)
        .arg(&config_file.path)
        .spawn()?;

    Ok(())
}
