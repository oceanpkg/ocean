//! Packaging and unpackaging drops.

use crate::drop::Manifest;
use flate2::{Compression, GzBuilder};
use std::{
    ffi::OsString,
    fs::{self, File},
    io::{self, Read, Seek, SeekFrom},
    path::{self, Path, PathBuf},
};

/// A package drop that can be `ship`ped.
#[derive(Debug)]
pub struct Package<Path = PathBuf> {
    /// Where the package resides.
    pub path: Path,
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

type TarBuilder<'a> = tar::Builder<flate2::write::GzEncoder<&'a mut File>>;

fn append_header(
    tar: &mut TarBuilder,
    tar_path: &Path, // The relative path within the tar file
    file: &mut File,
) -> io::Result<()> {
    let mut header = tar::Header::new_gnu();
    header.set_path(tar_path)?;
    header.set_metadata(&file.metadata()?);
    header.set_cksum();
    tar.append(&header, file)
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
            manifest_path_buf = current_dir.join(Manifest::FILE_NAME);
            &manifest_path_buf
        }
    };

    let mut manifest_file = File::open(manifest_path)?;
    let manifest = {
        let mut buf = String::with_capacity(128);
        manifest_file.read_to_string(&mut buf)?;
        manifest_file.seek(SeekFrom::Start(0))?;

        Manifest::parse_toml(&buf).map_err(|error| {
            io::Error::new(io::ErrorKind::InvalidData, error)
        })?
    };

    let drop_name = &manifest.meta.name;
    let drop_version = &manifest.meta.version;

    let tar_name = format!("{}.tar.gz", drop_name);
    let tmp_name = format!(".{}", tar_name);

    let output_dir = output_dir.unwrap_or(current_dir);
    let tar_path = output_dir.join(&tar_name);
    let tmp_path = output_dir.join(&tmp_name);

    // TODO: Change to `trace!`
    println!("Packaging \"{}\"", tar_path.display());

    fs::DirBuilder::new().recursive(true).create(&output_dir)?;

    let mut tmp_archive = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&tmp_path)?;

    let gz = GzBuilder::new()
        .filename(tar_name)
        .write(&mut tmp_archive, Compression::best());

    let mut tar = tar::Builder::new(gz);
    let tar_dir = OsString::from(format!(
        "{}@{}{}",
        drop_name,
        drop_version,
        path::MAIN_SEPARATOR
    ));

    for relative_path in manifest.files() {
        let full_path = current_dir.join(&relative_path);

        let mut tar_path = tar_dir.clone();
        tar_path.push(&relative_path);

        append_header(
            &mut tar,
            tar_path.as_ref(),
            &mut File::open(&full_path)?,
        )?;
    }

    // Append manifest after iterating over its files to reuse `tar_dir`.
    {
        let mut manifest_tar_path = tar_dir;
        manifest_tar_path.push(Manifest::FILE_NAME);
        append_header(
            &mut tar,
            manifest_tar_path.as_ref(),
            &mut manifest_file,
        )?;
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
