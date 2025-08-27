use std::path::Path;

use crate::Result;
use crate::StripPrefixError;

pub(crate) fn strip_prefix(path: &Path, prefix: impl AsRef<Path>) -> Result<&Path> {
    let prefix = prefix.as_ref();
    Ok(path.strip_prefix(prefix).map_err(|_| StripPrefixError {
        path: path.to_owned(),
        prefix: prefix.to_owned(),
    })?)
}
