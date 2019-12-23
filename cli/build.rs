use std::process;

fn main() {
    emit_git_rev();
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
