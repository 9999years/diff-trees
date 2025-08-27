use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;

/// An error encountered while hashing a file to determine if it changed.
#[derive(Debug)]
pub struct HashError {
    pub(crate) path: PathBuf,
    pub(crate) inner: std::io::Error,
}

impl HashError {
    /// The path that caused this error.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to hash file `{}`: {}",
            self.path.display(),
            self.inner
        )
    }
}

impl std::error::Error for HashError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}
