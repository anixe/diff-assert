use crate::Line;

#[derive(Debug)]
pub struct Hunk<'a> {
    pub(crate) old_start: usize,
    pub(crate) new_start: usize,
    pub(crate) inserted: usize,
    pub(crate) removed: usize,

    pub(crate) lines: Vec<Line<'a>>,
}
