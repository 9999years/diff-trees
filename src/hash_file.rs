use std::path::Path;

use crate::Result;
use crate::error::HashError;

pub(crate) fn hash_file(path: impl AsRef<Path>) -> Result<blake3::Hash> {
    let path = path.as_ref();
    tracing::debug!("Hashing {path:?}");
    Ok(blake3::Hasher::new()
        .update_mmap(path)
        .map_err(|inner| HashError {
            path: path.to_owned(),
            inner,
        })?
        .finalize())
}
