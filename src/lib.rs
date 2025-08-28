//! Diff two directory trees and format their results.
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
    /// Diff two paths.
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
            let candidate_metadata = candidate.metadata().map_err(|inner| MetadataError {
                path: candidate.clone(),
                inner,
            })?;

            entry.tag = candidate_is_same(
                removed_entry.path(),
                &removed_metadata,
                &candidate,
                &candidate_metadata,
            )?;
            entry.inserted = Some(PathInfo {
                metadata: candidate_metadata,
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
