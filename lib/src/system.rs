//! System utilities.

use std::{
    ffi::OsStr,
    io,
    process::{Child, Command, ExitStatus},
};

/// The tool for opening resources in the user's preferred application.
///
/// | Platform | Value |
/// | :------- | :---- |
/// | Linux    | `xdg-open` |
/// | macOS    | `/usr/bin/open |
/// | Windows  | `start` |
pub const OPEN_TOOL: &str = {
    cfg_if! {
        if #[cfg(target_os = "macos")] {
            let open = "/usr/bin/open";
        } else if #[cfg(target_os = "linux")] {
            let open = "xdg-open";
        } else if #[cfg(target_os = "windows")] {
            let open = "start";
        } else {
            compile_error!(
                "No known way of opening a resource on the current platform."
            );
        }
    };
    open
};

/// Opens `resources` in the user's preferred application.
pub fn open<R>(resources: &[R]) -> io::Result<()>
where
    R: AsRef<OsStr>,
{
    fn status_to_result(status: ExitStatus) -> io::Result<()> {
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Opening finished with status {}", status),
            ))
        }
    }

    // Do nothing if no resources were provided.
    if resources.is_empty() {
        return Ok(());
    }

    if cfg!(target_os = "macos") {
        // The tool supports providing multiple resources at once.
        let mut cmd = Command::new(OPEN_TOOL);
        cmd.args(resources);
        status_to_result(cmd.status()?)
    } else {
        // The tool needs to be spawned multiple times in order to open multiple
        // resources.

        // Spawn all processes before attempting to get their exit statuses.
        // This ensures we don't fail early if a single resource couldn't be
        // opened for any reason.
        let children: Vec<io::Result<Child>> = resources
            .iter()
            .map(|res| {
                let mut cmd = Command::new(OPEN_TOOL);
                cmd.arg(res);
                cmd.spawn()
            })
            .collect();

        for child in children {
            status_to_result(child?.wait()?)?;
        }

        Ok(())
    }
}
