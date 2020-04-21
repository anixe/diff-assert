use crate::{Hunk, Line, Processor};
use std::collections::VecDeque;
use std::io;

#[derive(Debug, Default)]
pub struct Context<'a> {
    pub start: Option<usize>,
    pub data: VecDeque<Line<'a>>,
    pub changed: bool,

    pub equaled: usize,
    pub removed: usize,
    pub inserted: usize,
}

impl<'a> Context<'a> {
    pub fn create_hunk(&mut self, removed: usize, inserted: usize) -> Option<Hunk<'a>> {
        let mut start = if let Some(start) = self.start {
            start
        } else {
            return None;
        };
        if start == 0 {
            start = 1;
        }
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

impl<'a> diffs::Diff for Processor<'a> {
    type Error = io::Error;

    fn equal(&mut self, old: usize, _new: usize, len: usize) -> Result<(), Self::Error> {
        let mut size = 0;

        if self.context.start.is_none() {
            self.context.start = Some(old);
        }

        for (i, j) in (old..old + len).zip(_new.._new + len) {
            if !self.context.changed {
                self.context
                    .data
                    .push_back(Line::line(i, j, &self.text1[i]));
                if size < self.context_radius {
                    self.context.equaled += 1;
                    size += 1;
                } else {
                    self.context.data.pop_front();
                    if let Some(ref mut start) = self.context.start {
                        *start += 1;
                    }
                }
            }

            if self.context.changed {
                if size < self.context_radius {
                    self.context
                        .data
                        .push_back(Line::line(i, j, &self.text1[i]));
                    self.context.equaled += 1;
                    size += 1;
                } else {
                    if let Some(hunk) = self.context.create_hunk(self.removed, self.inserted) {
                        self.result.push(hunk);
                    }

                    self.removed += self.context.removed;
                    self.inserted += self.context.inserted;
                    self.context = Context::default();
                    self.context
                        .data
                        .push_back(Line::line(i, j, &self.text1[i]));
                    size = 1;
                }
            }
        }

        Ok(())
    }

    fn delete(&mut self, old: usize, len: usize, _new: usize) -> Result<(), Self::Error> {
        if self.context.start.is_none() {
            self.context.start = Some(old);
        }

        for i in old..old + len {
            self.context.data.push_back(Line::remove(i, &self.text1[i]));
        }

        self.context.changed = true;
        self.context.removed += len;

        Ok(())
    }

    fn insert(&mut self, old: usize, new: usize, new_len: usize) -> Result<(), Self::Error> {
        if self.context.start.is_none() {
            self.context.start = Some(old);
        }

        for i in new..new + new_len {
            self.context.data.push_back(Line::insert(i, &self.text2[i]));
        }

        self.context.changed = true;
        self.context.inserted += new_len;

        Ok(())
    }

    fn replace(
        &mut self,
        old: usize,
        old_len: usize,
        new: usize,
        new_len: usize,
    ) -> Result<(), Self::Error> {
        if self.context.start.is_none() {
            self.context.start = Some(old);
        }

        for (i, j) in (old..old + old_len).zip(new..new + old_len) {
            let j = if j < (new + new_len) { Some(j) } else { None };
            self.context
                .data
                .push_back(Line::replace_remove(i, j, &self.text1[i]));
        }

        for (j, i) in (new..new + new_len).zip(old..old + new_len) {
            let i = if i < (old + old_len) { Some(i) } else { None };
            self.context
                .data
                .push_back(Line::replace_insert(i, j, &self.text2[j]));
        }

        self.context.changed = true;
        self.context.removed += old_len;
        self.context.inserted += new_len;

        Ok(())
    }

    fn finish(&mut self) -> Result<(), Self::Error> {
        if let Some(hunk) = self.context.create_hunk(self.removed, self.inserted) {
            self.result.push(hunk);
        }

        Ok(())
    }
}
