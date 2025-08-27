use std::fs::Metadata;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct PathInfo<'a> {
    pub(crate) metadata: Metadata,
    pub(crate) base: &'a Path,
}

impl<'a> PathInfo<'a> {
    /// Get the metadata for this path.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Get the comparison base for this path.
    ///
    /// This is one of the paths being diffed.
    ///
    /// The [`DiffEntry::relative`] field can be joined to the `base` to recover the original path.
    pub fn base(&self) -> &Path {
        &self.base
    }
}
