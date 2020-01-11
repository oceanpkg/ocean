use oceanpkg::Config;
use oceanpkg_shared::ext::*;
use std::{
    env::{self, consts::EXE_SUFFIX},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn run_external(
    config: &Config,
    cmd: &str,
    args: &clap::ArgMatches,
) -> crate::Result {
    use oceanpkg::env::*;

    let bin_dir = config.rt.bin_dir();
    let mut search_dirs = vec![bin_dir.clone()];

    if let Ok(path) = config.rt.path_var() {
        search_dirs.extend(env::split_paths(path));
    }

    let command_exe = find_executable(cmd, search_dirs).ok_or_else(|| {
        // TODO: List the possible subcommands and suggest a close match.
        failure::format_err!("No such subcommand: {}", cmd)
    })?;

    let mut command = Command::new(command_exe);

    if let Some(args) = args.values_of_os("") {
        command.args(args);
    }

    if let Ok(ocean) = config.rt.current_exe() {
        command.env(OCEAN, ocean);
    }
    command.env(OCEAN_BIN_DIR, &bin_dir);
    command.env(OCEAN_VERSION, env!("CARGO_PKG_VERSION"));

    // TODO: Turn into a custom error with a better error message.
    let error = command.spawn_replace();

    return Err(error.into());
}

#[cfg(unix)]
fn find_executable(cmd: &str, dirs: Vec<PathBuf>) -> Option<PathBuf> {
    find_executable_simple(cmd, dirs)
}

#[cfg(windows)]
fn find_executable(cmd: &str, dirs: Vec<PathBuf>) -> Option<PathBuf> {
    use std::ffi::OsString;

    if let Some(path_ext) = env::var_os("PATHEXT") {
        for ext in env::split_paths(&path_ext) {
            let mut file = OsString::from(format!("ocean-{}", cmd));
            file.push(ext);

            for dir in &dirs {
                let file = dir.join(&file);
                if is_executable(&file) {
                    return Some(file);
                }
            }
        }
        None
    } else {
        find_executable_simple(cmd, dirs)
    }
}

fn find_executable_simple(cmd: &str, dirs: Vec<PathBuf>) -> Option<PathBuf> {
    let file_name = format!("ocean-{}{}", cmd, EXE_SUFFIX);
    dirs.into_iter()
        .map(|dir| dir.pushing(&file_name))
        .find(|file| is_executable(file))
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::prelude::*;
    fs::metadata(path)
        .map(|metadata| {
            metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
        })
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}
