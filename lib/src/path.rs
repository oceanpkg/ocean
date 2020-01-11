use std::{
    env,
    ffi::OsStr,
    io, iter,
    path::{Path, PathBuf},
};
use shared::ext::*;

pub fn resolve_exe(
    exe: &Path,
    path_var: Option<&OsStr>,
) -> io::Result<PathBuf> {
    if let Some(path_var) = path_var {
        let candidates = env::split_paths(path_var).flat_map(|path| {
            let candidate = path.pushing(exe);
            let with_exe = if env::consts::EXE_EXTENSION.is_empty() {
                None
            } else {
                Some(candidate.with_extension(env::consts::EXE_EXTENSION))
            };
            iter::once(candidate).chain(with_exe)
        });
        for candidate in candidates {
            if candidate.is_file() {
                return candidate.canonicalize();
            }
        }
    }
    exe.canonicalize()
}
