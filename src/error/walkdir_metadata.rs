use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;

/// An error encountered while accessing a [`Path`]'s [`Path::metadata`] while walking a directory
/// tree.
#[derive(Debug)]
pub struct WalkDirMetadataError {
    pub(crate) path: PathBuf,
    pub(crate) inner: walkdir::Error,
}

impl WalkDirMetadataError {
    /// The path that caused this error.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Display for WalkDirMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to query metadata for `{}`: {}",
            self.path.display(),
            self.inner
        )
    }
}

impl std::error::Error for WalkDirMetadataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
