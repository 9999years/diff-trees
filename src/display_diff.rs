use std::fmt::Display;

use crate::Diff;
use crate::display_diff_opts::DisplayDiffOpts;

/// A [`Display`]able [`Diff`] combined with [`DisplayDiffOpts`].
pub struct DisplayDiff<'a> {
    pub(crate) diff: &'a Diff<'a>,
    pub(crate) opts: DisplayDiffOpts,
}

impl<'a> Display for DisplayDiff<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.diff.entries {
            entry.fmt_with(f, &self.opts)?;
        }
        Ok(())
    }
}
