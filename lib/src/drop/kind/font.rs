use crate::drop::Metadata;

/// A package for a typeface with specific properties; e.g. bold, italic.
#[derive(Clone, Debug)]
pub struct Font {
    metadata: Metadata,
    file_name: String,
}

impl Font {
    /// Returns basic metadata for the drop.
    #[inline]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
