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

    pub counter: usize,
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

            counter: 0,
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
        if self.context.start.is_none() {
            self.context.start = Some(old);
        }

        self.context.counter = 0;
        for (i, j) in (old..old + len).zip(_new.._new + len) {
            if !self.context.changed {
                if self.context.counter < self.context_radius {
                    self.context
                        .data
                        .push_back(Line::line(i, j, &self.text1[i]));
                    self.context.equaled += 1;
                    self.context.counter += 1;
                }
                if self.context.counter >= self.context_radius {
                    self.context
                        .data
                        .push_back(Line::line(i, j, &self.text1[i]));
                    self.context.data.pop_front();
                    if let Some(ref mut start) = self.context.start {
                        *start += 1;
                    }
                    self.context.counter += 1;
                }
            }
            if self.context.changed {
                if self.context.counter < self.context_radius * 2 {
                    self.context
                        .data
                        .push_back(Line::line(i, j, &self.text1[i]));
                    self.context.equaled += 1;
                    self.context.counter += 1;
                }
                if self.context.counter == self.context_radius && len > self.context_radius * 2 {
                    if let Some(hunk) = self.context.to_hunk(self.removed, self.inserted) {
                        self.result.push(hunk);
                    }

                    let mut context = Context::new();
                    for _ in 0..self.context_radius {
                        context.data.push_back(Line::line(0, 0, ""));
                    }
                    context.counter = self.context_radius;
                    context.equaled = self.context_radius;
                    context.start = Some(i - 1);

                    self.removed += self.context.removed;
                    self.inserted += self.context.inserted;
                    self.context = context;
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

        for i in old..old + old_len {
            self.context.data.push_back(Line::remove(i, &self.text1[i]));
        }
        for i in new..new + new_len {
            self.context.data.push_back(Line::insert(i, &self.text2[i]));
        }
        self.context.changed = true;
        self.context.removed += old_len;
        self.context.inserted += new_len;

        Ok(())
    }

    fn finish(&mut self) -> Result<(), Self::Error> {
        if self.context.counter > self.context_radius {
            let truncation = self.context.counter - self.context_radius;
            if self.context.data.len() > truncation {
                let new_size = self.context.data.len() - truncation;
                self.context.equaled -= truncation;
                self.context.data.truncate(new_size);
            }
        }
        if let Some(hunk) = self.context.to_hunk(self.removed, self.inserted) {
            self.result.push(hunk);
        }

        Ok(())
    }
}
