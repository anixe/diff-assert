use crate::Line;

/// Contains group of differing lines wrapped by sequences of lines common to both files.
#[derive(Debug)]
pub struct Hunk<'a> {
    pub(crate) old_start: usize,
    pub(crate) new_start: usize,
    pub(crate) inserted: usize,
    pub(crate) removed: usize,
    pub(crate) lines: Vec<Line<'a>>,
}

impl<'a> Hunk<'a> {
    /// Old/left start line of a hunk
    pub fn old_start(&self) -> usize {
        self.old_start
    }
    /// New/right start line of a hunk
    pub fn new_start(&self) -> usize {
        self.new_start
    }
    /// How many lines were inserted
    pub fn inserted(&self) -> usize {
        self.inserted
    }
    /// How many lines were removed
    pub fn removed(&self) -> usize {
        self.removed
    }
    /// Slice of the lines sequence
    pub fn lines(&self) -> &[Line<'a>] {
        &self.lines
    }
}
