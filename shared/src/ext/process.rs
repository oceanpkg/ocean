use std::{io, process::Command};

/// Extended functionality for
/// [`Command`](https://doc.rust-lang.org/std/process/struct.Command.html).
pub trait CommandExt {
    /// Spawns `self`, replacing the calling program.
    ///
    /// Behavior:
    ///
    /// - Unix: simply calls [`exec`].
    /// - Windows: sets the ctrl-c handler to always return `TRUE` (forwarding
    ///   ctrl-c to the child), calls [`status`], and exits with the returned
    ///   code.
    ///
    /// [`exec`]:   https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.exec
    /// [`status`]: https://doc.rust-lang.org/std/process/struct.Command.html#method.status
    fn spawn_replace(&mut self) -> io::Error;
}

impl CommandExt for Command {
    #[cfg(unix)]
    #[inline]
    fn spawn_replace(&mut self) -> io::Error {
        use std::os::unix::process::CommandExt;
        self.exec()
    }

    #[cfg(windows)]
    #[inline]
    fn spawn_replace(&mut self) -> io::Error {
        use std::process::exit;
        use winapi::{
            shared::minwindef::{BOOL, DWORD, TRUE},
            um::consoleapi::SetConsoleCtrlHandler,
        };

        unsafe extern "system" fn ctrlc_handler(_: DWORD) -> BOOL {
            // Do nothing; let the child process handle it.
            TRUE
        }

        unsafe {
            // TODO: Consider warning about this function failing.
            SetConsoleCtrlHandler(Some(ctrlc_handler), TRUE);
        }

        match self.status() {
            Ok(status) => {
                let exit_code = match status.code() {
                    Some(code) => code,
                    None if status.success() => 0,
                    None => 1,
                };
                exit(exit_code);
            }
            Err(error) => error,
        }
    }
}
