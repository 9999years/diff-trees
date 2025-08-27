use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;

/// An error encountered while traversing the directory trees.
#[derive(Debug)]
pub struct TraverseError {
    pub(crate) path: PathBuf,
    pub(crate) inner: walkdir::Error,
}

impl TraverseError {
    /// The path that caused this error.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Display for TraverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to traverse `{}`: {}",
            self.path.display(),
            self.inner
        )
    }
}

impl std::error::Error for TraverseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
