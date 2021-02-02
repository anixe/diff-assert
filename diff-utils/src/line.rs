/// Contains one line represented by slice to the original/new file, its [`kind`](enum.LineKind.html)
/// and positions in both files.
#[derive(Debug, Clone)]
pub struct Line<'a> {
    pub(crate) kind: LineKind,
    pub(crate) inner: &'a str,
    pub(crate) old_pos: Option<usize>,
    pub(crate) new_pos: Option<usize>,
}

/// Line kind specifies what happened to it.
#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Copy)]
pub enum LineKind {
    /// It existed in original file but no more,
    Removed,
    /// It didnt exist in original file but now does,
    Inserted,
    /// It was removed in original file and replaced by another one,
    ReplaceRemoved,
    /// It was inserted to the new line by replacing another one,
    ReplaceInserted,
    /// Line exists in both files.
    Unchanged,
}

impl LineKind {
    pub(crate) fn invert(self) -> Self {
        match self {
            LineKind::Removed => LineKind::Inserted,
            LineKind::Inserted => LineKind::Removed,
            LineKind::ReplaceInserted => LineKind::ReplaceRemoved,
            LineKind::ReplaceRemoved => LineKind::ReplaceInserted,
            u => u,
        }
    }

    pub(crate) fn sign(&self) -> &str {
        match self {
            LineKind::ReplaceInserted | LineKind::Inserted => "+",
            LineKind::ReplaceRemoved | LineKind::Removed => "-",
            LineKind::Unchanged => " ",
        }
    }

    pub(crate) fn is_replaced(self) -> bool {
        matches!(self, LineKind::ReplaceInserted | LineKind::ReplaceRemoved)
    }
}

impl<'a> Line<'a> {
    pub(crate) fn insert(pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Inserted,
            inner,
            old_pos: None,
            new_pos: Some(pos),
        }
    }

    pub(crate) fn remove(pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Removed,
            inner,
            old_pos: Some(pos),
            new_pos: None,
        }
    }

    pub(crate) fn replace_insert(old_pos: Option<usize>, new_pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::ReplaceInserted,
            inner,
            old_pos,
            new_pos: Some(new_pos),
        }
    }

    pub(crate) fn replace_remove(old_pos: usize, new_pos: Option<usize>, inner: &'a str) -> Self {
        Line {
            kind: LineKind::ReplaceRemoved,
            inner,
            old_pos: Some(old_pos),
            new_pos,
        }
    }

    pub(crate) fn unchanged(old_pos: usize, new_pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Unchanged,
            inner,
            old_pos: Some(old_pos),
            new_pos: Some(new_pos),
        }
    }
}
