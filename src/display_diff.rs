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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use indoc::indoc;
    use testlib::TempTree;

    #[test]
    fn test_display() -> Result<()> {
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
            .unwrap()
            .dir("d")
            .unwrap()
            .file("d/1", "1")
            .unwrap()
            .file("d/2", "2")
            .unwrap();

        let diff = Diff::new(old.as_ref(), new.as_ref())?;

        assert_eq!(
            diff.to_string(),
            indoc!(
                r#"
                ~ b/1
                ~ b/2
                - c/
                + d/
                "#
            )
        );

        Ok(())
    }
}
