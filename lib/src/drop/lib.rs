use crate::drop::Metadata;

/// A package for a library of a given language.
#[derive(Clone, Debug)]
pub struct Lib {
    metadata: Metadata,
}

impl Lib {
    /// Returns basic metadata for the drop.
    #[inline]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
