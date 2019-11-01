use std::path::{Path, PathBuf};

/// Extended functionality for
/// [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html).
pub trait PathBufExt {
    /// Like `join`, only reusing the underlying buffer.
    fn pushing<P: AsRef<Path>>(self, path: P) -> PathBuf;
}

impl PathBufExt for PathBuf {
    fn pushing<P: AsRef<Path>>(mut self, path: P) -> PathBuf {
        self.push(path);
        self
    }
}
