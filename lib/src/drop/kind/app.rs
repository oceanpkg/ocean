use crate::drop::Metadata;

/// A package with a graphical interface.
#[derive(Clone, Debug)]
pub struct App {
    metadata: Metadata,
    file_name: String,
}

impl App {
    /// Returns basic metadata for the drop.
    #[inline]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
