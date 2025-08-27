use owo_colors::Style;

/// This is a local equivalent of the [`similar::DiffTag`][1] enum.
///
/// [1]: https://docs.rs/similar/latest/similar/enum.DiffTag.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffTag {
    /// An entry that is equal in both sides of the diff.
    Equal,
    /// An entry that is present in the 'old' side of the diff and absent in the 'new' side.
    Delete,
    /// An entry that is present in both sides of the diff, but with changed contents.
    Replace,
    /// An entry that is absent in the 'old' side of the diff and present in the 'new' side.
    Insert,
}

impl DiffTag {
    /// TODO: Should probably be customizable.
    pub(crate) fn style(&self) -> Style {
        match self {
            DiffTag::Equal => Style::new(),
            DiffTag::Delete => Style::new().red(),
            DiffTag::Replace => Style::new().yellow(),
            DiffTag::Insert => Style::new().green(),
        }
    }

    /// TODO: Should probably be customizable.
    pub(crate) fn marker(&self) -> char {
        match self {
            DiffTag::Equal => ' ',
            DiffTag::Delete => '-',
            DiffTag::Replace => '~',
            DiffTag::Insert => '+',
        }
    }
}
