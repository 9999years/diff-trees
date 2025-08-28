#[cfg(doc)]
use crate::Diff;
#[cfg(doc)]
use std::fmt::Display;

/// Options for [`Display`]ing a [`Diff`].
#[derive(Debug, Default, Clone)]
pub struct DisplayDiffOpts {
    pub(crate) color: bool,
}

impl DisplayDiffOpts {
    /// Create a new default [`DisplayDiffOpts`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Whether to enable terminal colors when displaying the diff.
    pub fn color(self, color: bool) -> Self {
        Self { color }
    }
}
