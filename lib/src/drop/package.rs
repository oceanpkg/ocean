//! Packaging and unpackaging drops.

use std::{
    io::{self, Seek, SeekFrom},
    fs::{self, File},
    path::{self, Path, PathBuf},
};
use flate2::{Compression, GzBuilder};
use crate::drop::Manifest;

/// A package drop that can be `ship`ped.
#[derive(Debug)]
pub struct Package {
    /// Where the package resides.
    pub path: PathBuf,
    /// The parsed manifest.
    pub manifest: Manifest,
    /// An open handle to the file for reading/writing.
    pub file: File,
}

impl Package {
    /// Packages a drop in the context of `current_dir`.
    ///
    /// The manifest is expected to be found at
    /// - `manifest_path` or
    /// - `current_dir/Ocean.toml`, if `manifest_path` is not provided
    pub fn create<A, B, C>(
        current_dir: A,
        manifest_path: Option<B>,
        output_dir: Option<C>,
    ) -> io::Result<Package>
    where
        A: AsRef<Path>,
        B: AsRef<Path>,
        C: AsRef<Path>,
    {
        package_impl(
            current_dir.as_ref(),
            manifest_path.as_ref().map(|p| p.as_ref()),
            output_dir.as_ref().map(|p| p.as_ref()),
        )
    }
}

fn package_impl(
    current_dir: &Path,
    manifest_path: Option<&Path>,
    output_dir: Option<&Path>,
) -> io::Result<Package> {
    let manifest_path_buf: PathBuf;
    let manifest_path = match manifest_path {
        Some(path) => path,
        None => {
            manifest_path_buf = current_dir.join("Ocean.toml");
            &manifest_path_buf
        },
    };

    let manifest = Manifest::read_toml_file(manifest_path)?;
    let tar_name = format!("{}.tar.gz", manifest.meta.name);
    let tmp_name = format!(".{}", tar_name);

    let output_dir = output_dir.unwrap_or(current_dir);
    let tar_path = output_dir.join(&tar_name);
    let tmp_path = output_dir.join(&tmp_name);

    // TODO: Change to `trace!`
    println!("Packaging \"{}\"", tar_path.display());

    fs::DirBuilder::new()
        .recursive(true)
        .create(&output_dir)?;

    let mut tmp_archive = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&tmp_path)?;

    let gz = GzBuilder::new()
        .filename(tar_name)
        .write(&mut tmp_archive, Compression::best());

    let mut tar = tar::Builder::new(gz);

    for file_path in manifest.files() {
        let mut header = tar::Header::new_gnu();

        let path = format!(
            "{name}@{version}{separator}{file}",
            name = manifest.meta.name,
            version = manifest.meta.version,
            separator = path::MAIN_SEPARATOR,
            file = file_path,
        );
        header.set_path(&path)?;

        let full_path = current_dir.join(&file_path);
        let mut file = File::open(&full_path)?;

        let metadata = file.metadata()?;
        header.set_metadata(&metadata);

        header.set_cksum();
        tar.append(&header, &mut file)?;
    }

    let gz = tar.into_inner()?;
    gz.finish()?;

    fs::rename(&tmp_path, &tar_path)?;

    // Set the internal cursor to 0 to allow for subsequent reading.
    tmp_archive.seek(SeekFrom::Start(0))?;

    Ok(Package {
        path: tar_path,
        manifest,
        file: tmp_archive,
    })
}
