use std::path::Path;
use std::path::PathBuf;

use iddqd::IdOrdItem;
use iddqd::id_upcast;
use owo_colors::Style;

use crate::DiffTag;
use crate::DisplayDiffOpts;
use crate::PathInfo;

/// A single entry in a diff, identified by a path relative to the diff base directory.
#[derive(Debug, Clone)]
pub struct DiffEntry<'a> {
    pub(crate) relative: PathBuf,
    pub(crate) tag: DiffTag,
    pub(crate) deleted: Option<PathInfo<'a>>,
    pub(crate) inserted: Option<PathInfo<'a>>,
}

impl<'a> IdOrdItem for DiffEntry<'a> {
    type Key<'b>
        = &'b Path
    where
        Self: 'b;

    fn key(&self) -> Self::Key<'_> {
        self.relative.as_path()
    }

    id_upcast! {}
}

impl<'a> DiffEntry<'a> {
    /// The path of this entry, relative to the old and new paths being diffed.
    pub fn relative(&self) -> &Path {
        &self.relative
    }

    /// The change made; was this path removed, inserted, or changed?
    pub fn tag(&self) -> DiffTag {
        self.tag
    }

    /// Information for the old path, if any.
    pub fn deleted(&self) -> Option<&PathInfo<'a>> {
        self.deleted.as_ref()
    }

    /// Information for the new path, if any.
    pub fn inserted(&self) -> Option<&PathInfo<'a>> {
        self.inserted.as_ref()
    }

    pub(crate) fn is_dir(&self) -> bool {
        self.inserted
            .as_ref()
            .or(self.deleted.as_ref())
            .map(|info| info.metadata.is_dir())
            .unwrap_or(false)
    }

    pub(crate) fn format_path(&self) -> String {
        let mut ret = self.relative.display().to_string();
        if self.is_dir() {
            ret.push('/');
        }
        ret
    }

    fn styled(&self, style: Style) -> owo_colors::Styled<String> {
        style.style(format!("{} {}", self.tag.marker(), self.format_path()))
    }

    pub(crate) fn fmt_with(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayDiffOpts,
    ) -> std::fmt::Result {
        let style = if opts.color {
            self.tag.style()
        } else {
            Style::new()
        };

        match self.tag {
            DiffTag::Equal => {}
            DiffTag::Delete => {
                writeln!(f, "{}", self.styled(style))?;
            }
            DiffTag::Replace => {
                // Directory entries are not very useful for me. This should probably also be
                // customizable.
                if !self.is_dir() {
                    writeln!(f, "{}", self.styled(style))?;
                }
            }
            DiffTag::Insert => {
                writeln!(f, "{}", self.styled(style))?;
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn as_pair(&self) -> (&Path, DiffTag) {
        (self.relative(), self.tag())
    }
}
