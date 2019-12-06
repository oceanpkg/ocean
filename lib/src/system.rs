//! System utilities.

use std::{
    ffi::OsStr,
    process::Command,
};

/// Returns a [`Command`] for opening a `resource` in the user's preferred
/// application.
///
/// [`Command`]: https://doc.rust-lang.org/std/process/struct.Command.html
pub fn open_cmd<R: AsRef<OsStr>>(resource: R) -> Command {
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            let open = "/usr/bin/open";
        } else if #[cfg(target_os = "linux")] {
            let open = "xdg-open";
        } else {
            compile_error!(
                "No known way of opening a resource on the current platform."
            );
        }
    };
    let mut cmd = Command::new(open);
    cmd.arg("--").arg(resource);
    cmd
}
