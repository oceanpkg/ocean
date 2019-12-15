use std::process;

fn main() {
    emit_git_rev();
}

macro_rules! cargo_warn {
    ($fmt:literal $($args:tt)*) => {
        println!(concat!("cargo:warning=", $fmt) $($args)*)
    };
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
                    println!("cargo:rustc-env=OCEAN_GIT_REV={}", rev);
                }
            },
            Err(error) => cargo_warn!("Could not parse git hash: {}", error),
        },
        Err(error) => cargo_warn!("Could not run `git`: {}", error),
    }
}
