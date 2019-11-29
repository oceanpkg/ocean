use std::{
    path::PathBuf,
    process::Command,
};
use crate::{
    drop::{
        Metadata,
        name::DropQuery,
    },
    install::InstallTarget,
};

/// A package that can be executed; e.g. CLI tool or script.
#[derive(Clone, Debug)]
pub struct Exe {
    metadata: Metadata,
    bin_name: String,
}

impl Exe {
    /// Returns an executable matching `query`, installed for `target`.
    pub fn installed(query: DropQuery, target: &InstallTarget) -> Result<Self, ()> {
        unimplemented!("TODO: Find installation of {:?} for {:?}", query, target)
    }

    /// Returns basic metadata for the drop.
    #[inline]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns the name of the binary executable.
    #[inline]
    pub fn bin_name(&self) -> &str {
        &self.bin_name
    }

    /// Returns the path of the drop's executable binary, if one exists.
    pub fn bin_path<'t>(
        &self,
        target: &'t InstallTarget,
    ) -> Result<PathBuf, FindError<'t>> {
        unimplemented!(
            "TODO: Find {:?} binary for {:?}",
            self.metadata.name,
            target,
        )
    }

    /// Returns a `Command` instance suitable for running the drop's executable
    /// binary, if one exists.
    pub fn command<'t>(
        &self,
        target: &'t InstallTarget
    ) -> Result<Command, FindError<'t>> {
        self.bin_path(target).map(Command::new)
    }
}

/// An error returned when the binary for an executable drop cannot be found.
#[derive(Debug)]
pub struct FindError<'a> {
    target: &'a InstallTarget,
}
