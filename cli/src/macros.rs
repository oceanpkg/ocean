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
