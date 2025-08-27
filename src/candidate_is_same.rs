use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::DiffTag;
use crate::Result;
use crate::hash_file::hash_file;

pub(crate) fn candidate_is_same(
    removed_path: &Path,
    removed_metadata: &Metadata,
    candidate_path: &Path,
    candidate_metadata: &Metadata,
) -> Result<DiffTag> {
    Ok(
        if (candidate_metadata.dev(), candidate_metadata.ino())
            == (removed_metadata.dev(), removed_metadata.ino())
        {
            DiffTag::Equal
        } else if removed_metadata.is_dir()
            || candidate_metadata.is_dir()
            || candidate_metadata.len() != removed_metadata.len()
            || hash_file(removed_path)? != hash_file(candidate_path)?
        {
            DiffTag::Replace
        } else {
            DiffTag::Equal
        },
    )
}
