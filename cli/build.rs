use std::{
    env,
    process,
};

fn main() {
    emit_target_info();
    emit_git_rev();
}

/// Make the target's system info available to the build.
fn emit_target_info() {
    let mut target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_os = match target_os.as_str() {
        "macos" => "macOS",
        _ => {
            target_os[..1].make_ascii_lowercase();
            &target_os
        },
    };
    cargo_emit::rustc_env!("OCEAN_TARGET_OS", "{}", target_os);

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    cargo_emit::rustc_env!("OCEAN_TARGET_ARCH", "{}", target_arch);
}

/// Make the current git revision hash available to the build.
fn emit_git_rev() {
    let git_output = process::Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output();
    match git_output {
        Ok(git_output) => match String::from_utf8(git_output.stdout) {
            Ok(rev) => {
                let rev = rev.trim();
                if !rev.is_empty() {
                    cargo_emit::rustc_env!("OCEAN_GIT_REV", "{}", rev);
                }
            },
            Err(error) => {
                cargo_emit::warning!("Could not parse git hash: {}", error);
            },
        },
        Err(error) => {
            cargo_emit::warning!("Could not run `git`: {}", error)
        },
    }
}
