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
    let mut search_dirs = vec![config.rt.bin_dir()];

    if let Some(path) = env::var_os("PATH") {
        search_dirs.extend(env::split_paths(&path));
    }

    let command_exe = find_executable(cmd, search_dirs).ok_or_else(|| {
        // TODO: List the possible subcommands and suggest a close match.
        failure::format_err!("No such subcommand: {}", cmd)
    })?;

    let mut command = Command::new(command_exe);

    if let Some(args) = args.values_of_os("") {
        command.args(args);
    }

    // TODO: Turn into a custom error with a better error message.
    return Err(command.spawn_replace().into());
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
