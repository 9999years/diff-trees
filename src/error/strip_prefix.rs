use std::fmt::Display;
use std::path::Path;
use std::path::PathBuf;

/// An error encountered while removing a prefix from a [`Path`].
#[derive(Debug)]
pub struct StripPrefixError {
    pub(crate) path: PathBuf,
    pub(crate) prefix: PathBuf,
}

impl StripPrefixError {
    /// The path that caused this error.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The prefix that the path was expected to have.
    pub fn prefix(&self) -> &Path {
        &self.prefix
    }
}

impl Display for StripPrefixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Path does not have prefix {}: {}",
            self.prefix.display(),
            self.path.display(),
        )
    }
}

impl std::error::Error for StripPrefixError {}
