use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;

/// An error encountered while accessing a [`Path`]'s [`Path::metadata`].
#[derive(Debug)]
pub struct MetadataError {
    pub(crate) path: PathBuf,
    pub(crate) inner: std::io::Error,
}

impl MetadataError {
    /// The path that caused this error.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to query metadata for `{}`: {}",
            self.path.display(),
            self.inner
        )
    }
}

impl std::error::Error for MetadataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
