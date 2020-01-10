//! Archiving utilities.

use flate2::read::GzDecoder;
use std::{io, path::Path};

/// Reads `tarball` as a `.tar.gz` file and unpacks it to `path`.
///
/// Because `GzDecoder` uses a buffered reader internally, this is appropriate
/// to call on `File`s.
pub fn unpack_tarball<R, P>(tarball: R, path: P) -> io::Result<()>
where
    R: io::Read,
    P: AsRef<Path>,
{
    let decoder = GzDecoder::new(tarball);
    println!("{:#?}", decoder.header());
    tar::Archive::new(decoder).unpack(path)
}
