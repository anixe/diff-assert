//! Contains [`Context`](struct.Context.html)

use crate::{Hunk, Line};
use std::collections::VecDeque;

#[derive(Debug, Default)]
pub(crate) struct Context<'a> {
    pub start: Option<usize>,
    pub data: VecDeque<Line<'a>>,
    pub changed: bool,

    pub equaled: usize,
    pub removed: usize,
    pub inserted: usize,
}

impl<'a> Context<'a> {
    pub fn create_hunk(&mut self, removed: usize, inserted: usize) -> Option<Hunk<'a>> {
        let start = self.start?;
        if self.changed {
            let mut data = VecDeque::new();
            data.append(&mut self.data);
            Some(Hunk {
                old_start: start,
                removed: self.equaled + self.removed,
                new_start: start + inserted - removed,
                inserted: self.equaled + self.inserted,
                lines: data.into_iter().collect(),
            })
        } else {
            None
        }
    }
}
