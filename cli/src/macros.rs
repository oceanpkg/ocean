/// Prints a formatted message to [`stderr`] indicating that an error has
/// occurred.
// TODO: Switch to `log::error!`.
macro_rules! error {
    ($fmt:literal $($args:tt)*) => { {
        eprintln!(concat!("error: ", $fmt) $($args)*);
    } };
}

/// Prints a message to [`stderr`] and exits the process with an exit code of 1.
///
/// If an identifier is passed, it will be printed using [`fmt::Display`].
///
/// [`stderr`]: https://en.wikipedia.org/wiki/Standard_streams#Standard_error_(stderr)
/// [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
macro_rules! exit_error {
    ($fmt:literal $($args:tt)*) => { {
        eprintln!($fmt $($args)*);
        std::process::exit(1);
    } };
    ($error:expr) => {
        exit_error!("{}", $error)
    };
}

/// Analogous to what [`println!`] is for [`print!`], but for [`format!`].
///
/// [`println!`]: https://doc.rust-lang.org/std/macro.println.html
/// [`print!`]:   https://doc.rust-lang.org/std/macro.print.html
/// [`format!`]:  https://doc.rust-lang.org/std/macro.format.html
macro_rules! formatln {
    ($fmt:literal $($args:tt)*) => {
        format!(concat!($fmt, newline!()) $($args)*)
    };
}

#[cfg(unix)]
macro_rules! newline {
    () => {
        "\n"
    };
}

#[cfg(windows)]
macro_rules! newline {
    () => {
        "\r\n"
    };
}
