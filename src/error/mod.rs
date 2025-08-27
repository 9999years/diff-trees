use std::fmt::Display;

mod hash;
mod metadata;
mod strip_prefix;
mod traverse;
mod walkdir_metadata;

pub use hash::HashError;
pub use metadata::MetadataError;
pub use strip_prefix::StripPrefixError;
pub use traverse::TraverseError;
pub use walkdir_metadata::WalkDirMetadataError;

/// An error encountered while diffing two paths.
#[derive(Debug)]
pub enum Error {
    /// An error encountered while traversing the directory trees.
    Traverse(TraverseError),
    /// An error encountered while accessing a [`Path`]'s [`Path::metadata`].
    Metadata(MetadataError),
    /// An error encountered while accessing a [`Path`]'s [`Path::metadata`] while walking a
    /// directory tree.
    WalkDirMetadata(WalkDirMetadataError),
    /// An error encountered while hashing a file to determine if it changed.
    Hash(HashError),
    /// An error encountered while removing a prefix from a [`Path`].
    StripPrefix(StripPrefixError),
}

/// A [`std::result::Result`] produced by diffing two paths.
pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Traverse(inner) => inner.fmt(f),
            Error::Metadata(inner) => inner.fmt(f),
            Error::WalkDirMetadata(inner) => inner.fmt(f),
            Error::Hash(inner) => inner.fmt(f),
            Error::StripPrefix(inner) => inner.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Traverse(inner) => inner.source(),
            Error::Metadata(inner) => inner.source(),
            Error::WalkDirMetadata(inner) => inner.source(),
            Error::Hash(inner) => inner.source(),
            Error::StripPrefix(inner) => inner.source(),
        }
    }
}

impl From<TraverseError> for Error {
    fn from(value: TraverseError) -> Self {
        Self::Traverse(value)
    }
}

impl From<MetadataError> for Error {
    fn from(value: MetadataError) -> Self {
        Self::Metadata(value)
    }
}

impl From<WalkDirMetadataError> for Error {
    fn from(value: WalkDirMetadataError) -> Self {
        Self::WalkDirMetadata(value)
    }
}

impl From<HashError> for Error {
    fn from(value: HashError) -> Self {
        Self::Hash(value)
    }
}

impl From<StripPrefixError> for Error {
    fn from(value: StripPrefixError) -> Self {
        Self::StripPrefix(value)
    }
}
