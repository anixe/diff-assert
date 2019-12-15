mod hunk;
mod line;
mod processor;

pub use self::hunk::Hunk;
pub use self::line::Line;
pub use self::processor::Processor;
use std::{collections::VecDeque, io};

pub fn diff_hunks<'a>(
    text1: &'a [String],
    text2: &'a [String],
    context_radius: usize,
) -> io::Result<Vec<Hunk<'a>>> {
    let mut processor = Processor::new(&text1, &text2, context_radius);
    {
        let mut replace = diffs::Replace::new(&mut processor);
        diffs::patience::diff(&mut replace, text1, 0, text1.len(), text2, 0, text2.len())?;
    }
    Ok(processor.result())
}

pub fn diff(text1: &[String], text2: &[String], context_radius: usize) -> io::Result<Vec<String>> {
    let result = diff_hunks(text1, text2, context_radius)?
        .into_iter()
        .map(|hunk| format!("{}", hunk))
        .collect();
    Ok(result)
}

#[derive(Debug)]
pub struct Context<'a> {
    pub start: Option<usize>,
    pub data: VecDeque<Line<'a>>,
    pub changed: bool,

    pub equaled: usize,
    pub removed: usize,
    pub inserted: usize,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Self {
            start: None,
            data: VecDeque::new(),
            changed: false,

            equaled: 0,
            removed: 0,
            inserted: 0,
        }
    }

    pub fn to_hunk(&mut self, removed: usize, inserted: usize) -> Option<Hunk<'a>> {
        let mut start = if let Some(start) = self.start {
            start
        } else {
            return None;
        };
        if start == 0 {
            start = 1;
        }
        let mut hunk = None;
        if self.changed {
            let mut data = VecDeque::new();
            data.append(&mut self.data);
            hunk = Some(Hunk {
                old_start: start,
                removed: self.equaled + self.removed,
                new_start: start + inserted - removed,
                inserted: self.equaled + self.inserted,
                lines: data.into_iter().collect(),
            });
        }
        hunk
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
                self.context.data.push_back(Line::line(i, j, &self.text1[i]));
                if size < self.context_radius {
                    self.context.equaled += 1;
                    size += 1;
                }
                else {
                    self.context.data.pop_front();
                    if let Some(ref mut start) = self.context.start {
                        *start += 1;
                    }
                }
            }

            if self.context.changed {
                if size < self.context_radius {
                    self.context.data.push_back(Line::line(i, j, &self.text1[i]));
                    self.context.equaled += 1;
                    size += 1;
                }
                else {
                    if let Some(hunk) = self.context.to_hunk(self.removed, self.inserted) {
                        self.result.push(hunk);
                    }

                    self.removed += self.context.removed;
                    self.inserted += self.context.inserted;
                    self.context = Context::new();
                    self.context.data.push_back(Line::line(i, j, &self.text1[i]));
                    size = 1;
                }
            }
        }

        Ok(())
    }

    fn delete(&mut self, old: usize, len: usize) -> Result<(), Self::Error> {
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
            self.context.data.push_back(Line::replace_remove(i, j, &self.text1[i]));
        }

        for (j, i) in (new..new + new_len).zip(old..old + new_len) {
            let i = if i < (old + old_len) { Some(i) } else { None };
            self.context.data.push_back(Line::replace_insert(i, j, &self.text2[j]));
        }

        self.context.changed = true;
        self.context.removed += old_len;
        self.context.inserted += new_len;

        Ok(())
    }

    fn finish(&mut self) -> Result<(), Self::Error> {
        if let Some(hunk) = self.context.to_hunk(self.removed, self.inserted) {
            self.result.push(hunk);
        }

        Ok(())
    }
}
