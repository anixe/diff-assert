use crate::Hunk;
use crate::{Context, Line};
use std::io;

#[derive(Debug)]
pub struct Processor<'a> {
    pub(crate) text1: &'a [&'a str],
    pub(crate) text2: &'a [&'a str],

    pub(crate) context_radius: usize,
    pub(crate) inserted: usize,
    pub(crate) removed: usize,

    pub(crate) context: Context<'a>,
    pub(crate) result: Vec<Hunk<'a>>,
}

impl<'a> Processor<'a> {
    pub fn new(text1: &'a [&'a str], text2: &'a [&'a str], context_radius: usize) -> Self {
        Self {
            text1,
            text2,

            context_radius,
            inserted: 0,
            removed: 0,

            context: Context::default(),
            result: Vec::new(),
        }
    }

    pub fn result(self) -> Vec<Hunk<'a>> {
        self.result
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
                    .push_back(Line::unchanged(i, j, &self.text1[i]));
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
                /*
                We want * 2 in case next hunk would be adjacent to the current one.
                 */
                if size < self.context_radius * 2 {
                    self.context
                        .data
                        .push_back(Line::unchanged(i, j, &self.text1[i]));
                    self.context.equaled += 1;
                    size += 1;
                } else {
                    // But if there are more unchanged lines between two changes than context_radius * 2,
                    // then we want to split hunk into smaller.
                    let diff = size - self.context_radius;

                    let at = self.context.data.len() - diff;
                    let mut removed = self.context.data.split_off(at);
                    self.context.equaled -= diff;

                    if let Some(hunk) = self.context.create_hunk(self.removed, self.inserted) {
                        self.result.push(hunk);
                    }

                    self.removed += self.context.removed;
                    self.inserted += self.context.inserted;

                    removed.pop_front();
                    self.context = Context::default();
                    self.context.start = Some(i - removed.len());
                    self.context.equaled += removed.len() + 1;
                    size = removed.len() + 1;
                    self.context.data.extend(removed.into_iter());
                    self.context
                        .data
                        .push_back(Line::unchanged(i, j, &self.text1[i]));
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
