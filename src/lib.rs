//! Diff two directory trees based on their contents and format the resulting diff.
//!
//! Construct a diff with [`diff_treesets`], which produces a [`Diff`], which can be formatted or
//! inspected.

#![deny(missing_docs)]

use std::fmt::Display;
use std::ops::Deref;
use std::path::Path;

use iddqd::IdOrdMap;
use walkdir::WalkDir;

mod candidate_is_same;
mod diff_entry;
mod diff_tag;
mod display_diff;
mod display_diff_opts;
mod error;
mod hash_file;
mod path_info;
mod strip_prefix;

pub use diff_entry::DiffEntry;
pub use diff_tag::DiffTag;
pub use display_diff_opts::DisplayDiffOpts;
pub use error::Error;
pub use error::HashError;
pub use error::MetadataError;
pub use error::Result;
pub use error::StripPrefixError;
pub use error::TraverseError;
pub use error::WalkDirMetadataError;

use candidate_is_same::candidate_is_same;
use display_diff::DisplayDiff;
use path_info::PathInfo;
use strip_prefix::strip_prefix;

/// A diff of trees in terms of relative paths.
#[derive(Debug)]
pub struct Diff<'a> {
    entries: IdOrdMap<DiffEntry<'a>>,
}

impl<'a> Deref for Diff<'a> {
    type Target = IdOrdMap<DiffEntry<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<'a> IntoIterator for Diff<'a> {
    type Item = DiffEntry<'a>;

    type IntoIter = iddqd::id_ord_map::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a Diff<'a> {
    type Item = &'a DiffEntry<'a>;

    type IntoIter = iddqd::id_ord_map::Iter<'a, DiffEntry<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.entries).into_iter()
    }
}

impl<'a> Diff<'a> {
    /// Diff two directory trees.
    ///
    /// Paths are compared for equality with [`blake3`], which has good performance characteristics
    /// in my testing. In the future, I hope to [add pluggable comparators][issue-2], [rename
    /// detection][issue-9], and the [ability to produce a text diff of the compared
    /// files][issue-3].
    ///
    /// Note that directory entries which appear in both trees are considered to be
    /// [`DiffTag::Replace`]d, but this does not account for whether or not their contents have
    /// changed. A future version of this library may do something more intuitive in this case.
    ///
    /// [issue-2]: https://github.com/9999years/diff-trees/issues/2
    /// [issue-9]: https://github.com/9999years/diff-trees/issues/9
    /// [issue-3]: https://github.com/9999years/diff-trees/issues/3
    pub fn new(old: &'a Path, new: &'a Path) -> Result<Self> {
        let mut diff = Self {
            entries: IdOrdMap::new(),
        };

        diff.walk_removed_tree(old, new)?;
        diff.walk_added_tree(new)?;

        Ok(diff)
    }

    fn walk_removed_tree(&mut self, old: &'a Path, new: &'a Path) -> Result<()> {
        let walker = WalkDir::new(old).follow_links(true);
        let mut iterator = walker.into_iter();

        loop {
            let removed_entry = match iterator.next() {
                Some(entry) => entry.map_err(|inner| {
                    Error::Traverse(TraverseError {
                        path: old.to_path_buf(),
                        inner,
                    })
                }),
                None => break,
            }?;

            if removed_entry.depth() == 0 {
                continue;
            }

            let relative = strip_prefix(removed_entry.path(), old)?.to_path_buf();

            let removed_metadata =
                removed_entry
                    .metadata()
                    .map_err(|inner| WalkDirMetadataError {
                        path: removed_entry.path().to_owned(),
                        inner,
                    })?;

            let mut entry = DiffEntry {
                relative,
                tag: DiffTag::Delete,
                deleted: None,
                inserted: None,
            };

            let candidate = new.join(&entry.relative);
            let candidate_metadata = match candidate.metadata() {
                Ok(metadata) => Some(metadata),
                Err(err) => {
                    if err.kind() == std::io::ErrorKind::NotFound {
                        None
                    } else {
                        return Err(MetadataError {
                            path: candidate.clone(),
                            inner: err,
                        }
                        .into());
                    }
                }
            };

            entry.tag = match candidate_metadata.as_ref() {
                Some(candidate_metadata) => candidate_is_same(
                    removed_entry.path(),
                    &removed_metadata,
                    &candidate,
                    candidate_metadata,
                )?,
                None => DiffTag::Delete,
            };

            entry.inserted = candidate_metadata.map(|metadata| PathInfo {
                metadata,
                base: new,
            });

            if removed_entry.file_type().is_dir()
                && let DiffTag::Delete = entry.tag
            {
                // Don't recurse if a directory has been removed.
                iterator.skip_current_dir();
            }

            entry.deleted = Some(PathInfo {
                metadata: removed_metadata,
                base: old,
            });

            if let Some(overwritten) = self.entries.insert_overwrite(entry) {
                tracing::debug!(?overwritten, "Got two diff entries for a single path");
            }
        }
        Ok(())
    }

    fn walk_added_tree(&mut self, new: &'a Path) -> Result<()> {
        let walker = WalkDir::new(new).follow_links(true);
        let mut iterator = walker.into_iter();

        loop {
            let added_entry = match iterator.next() {
                Some(entry) => entry.map_err(|inner| {
                    Error::Traverse(TraverseError {
                        path: new.to_path_buf(),
                        inner,
                    })
                }),
                None => break,
            }?;

            if added_entry.depth() == 0 {
                continue;
            }

            let relative = strip_prefix(added_entry.path(), new)?.to_path_buf();

            match self.entries.get(relative.as_path()) {
                Some(diff_entry) => {
                    if let DiffTag::Delete = diff_entry.tag {
                        // Don't recurse if a directory has been removed.
                        iterator.skip_current_dir();
                        continue;
                    }
                }
                None => {
                    if added_entry.file_type().is_dir() {
                        iterator.skip_current_dir();
                    }

                    if let Some(overwritten) = self.entries.insert_overwrite(DiffEntry {
                        relative,
                        tag: DiffTag::Insert,
                        deleted: None,
                        inserted: Some(PathInfo {
                            metadata: added_entry.metadata().map_err(|inner| {
                                WalkDirMetadataError {
                                    path: added_entry.path().to_owned(),
                                    inner,
                                }
                            })?,
                            base: new,
                        }),
                    }) {
                        tracing::debug!(?overwritten, "Got two diff entries for a single path");
                    };
                }
            }
        }
        Ok(())
    }

    /// [`Display`] this diff with the given options.
    ///
    /// Note that [`Diff`] already implements [`Display`] with default options, but this method is
    /// available to enable colored output.
    pub fn display(&'a self, opts: DisplayDiffOpts) -> impl Display + 'a {
        DisplayDiff { diff: self, opts }
    }
}

/// Display the diff with default options (no ANSI colors).
impl<'a> Display for Diff<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(Default::default()).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testlib::TempTree;

    #[test]
    fn test_same_contents() -> Result<()> {
        let mut old = TempTree::new().unwrap();
        old.file("puppy", "puppy").unwrap();

        let mut new = TempTree::new().unwrap();
        new.file("puppy", "puppy").unwrap();

        let diff = Diff::new(old.as_ref(), new.as_ref())?;

        assert_eq!(
            (&diff)
                .into_iter()
                .map(DiffEntry::as_pair)
                .collect::<Vec<_>>(),
            vec![(Path::new("puppy"), DiffTag::Equal)]
        );

        Ok(())
    }

    #[test]
    fn test_different_contents() -> Result<()> {
        let mut old = TempTree::new().unwrap();
        old.file("puppy", "puppy").unwrap();

        let mut new = TempTree::new().unwrap();
        new.file("puppy", "doggy").unwrap();

        let diff = Diff::new(old.as_ref(), new.as_ref())?;

        assert_eq!(
            (&diff)
                .into_iter()
                .map(DiffEntry::as_pair)
                .collect::<Vec<_>>(),
            vec![(Path::new("puppy"), DiffTag::Replace)]
        );

        Ok(())
    }

    #[test]
    fn test_complex() -> Result<()> {
        let mut old = TempTree::new().unwrap();
        old.dir("a")
            .unwrap()
            .file("a/1", "1")
            .unwrap()
            .file("a/2", "2")
            .unwrap()
            .dir("b")
            .unwrap()
            .file("b/1", "1")
            .unwrap()
            .file("b/2", "2")
            .unwrap()
            .dir("c")
            .unwrap()
            .file("c/1", "1")
            .unwrap()
            .file("c/2", "2")
            .unwrap();

        let mut new = TempTree::new().unwrap();
        new.dir("a")
            .unwrap()
            .file("a/1", "1")
            .unwrap()
            .file("a/2", "2")
            .unwrap()
            .dir("b")
            .unwrap()
            .file("b/1", "1x")
            .unwrap()
            .file("b/2", "2x")
            .unwrap();

        let diff = Diff::new(old.as_ref(), new.as_ref())?;

        assert_eq!(
            (&diff)
                .into_iter()
                .map(DiffEntry::as_pair)
                .collect::<Vec<_>>(),
            vec![
                (Path::new("a"), DiffTag::Replace),
                (Path::new("a/1"), DiffTag::Equal),
                (Path::new("a/2"), DiffTag::Equal),
                (Path::new("b"), DiffTag::Replace),
                (Path::new("b/1"), DiffTag::Replace),
                (Path::new("b/2"), DiffTag::Replace),
                (Path::new("c"), DiffTag::Delete),
            ]
        );

        Ok(())
    }
}
