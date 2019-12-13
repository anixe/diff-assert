use crate::diff::line::{Line, LineKind};
use colored::*;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Hunk<'a> {
    pub(crate) old_start: usize,
    pub(crate) new_start: usize,
    pub(crate) inserted: usize,
    pub(crate) removed: usize,
    pub(crate) lines: Vec<Line<'a>>,
}

struct LineDiff<'a>(&'a Line<'a>, &'a Line<'a>);

impl<'a> Display for LineDiff<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let l = self
            .0
            .inner
            .chars()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>();
        let r = self
            .1
            .inner
            .chars()
            .map(|c| format!("{}", c))
            .collect::<Vec<_>>();

        let len = std::cmp::max(self.0.inner.len(), self.1.inner.len());
        let diff = crate::diff_hunks(&l, &r, len).unwrap();
        if diff.len() == 0 {
            return writeln!(f, "{}", self.1.fmt());
        }
        let hunk = &diff[0];

        let line = hunk
            .lines
            .iter()
            .filter(|l| l.kind != LineKind::Removed)
            .map(|letter| {
                if letter.kind == LineKind::Unchanged {
                    format!("{}", letter.inner.dimmed())
                } else if letter.kind == LineKind::Inserted {
                    format!("{}", letter.inner.reversed())
                } else {
                    unreachable!("Filters removed. Can't happen")
                }
            })
            .join("");

        let line = Line {
            inner: &line,
            ..self.1.clone()
        };

        let mut fmt = line.fmt();
        fmt.1 = true;
        writeln!(f, "{}", fmt)?;
        Ok(())
    }
}

impl<'a> Display for Hunk<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let line_diff = self.inserted == self.removed;

        let lines = self
            .lines
            .iter()
            .map(|line| ((line.new_pos, line.kind), (*line).clone()))
            .collect::<BTreeMap<(usize, LineKind), Line>>();

        let header = format!(
            "... ...   @@ -{},{} +{},{} @@",
            self.old_start, self.removed, self.new_start, self.inserted
        );
        writeln!(f, "{}", header.on_blue().black().dimmed())?;
        for line in self.lines.iter() {
            let new_pos = line.new_pos;
            let inverted_kind = line.kind.invert();

            let op = lines.get(&(new_pos, inverted_kind));

            if !line_diff || op.is_none() {
                writeln!(f, "{}", line.fmt())?;
                continue;
            }

            let op = op.unwrap(); // Previous if filters None;
            LineDiff(op, line).fmt(f)?
        }
        Ok(())
    }
}
