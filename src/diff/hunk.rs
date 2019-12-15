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
            .filter(|l| l.kind != LineKind::Removed && l.kind != LineKind::ReplaceRemoved)
            .map(|letter| {
                if letter.kind == LineKind::Unchanged {
                    format!("{}", letter.inner.dimmed())
                } else if letter.kind == LineKind::Inserted || letter.kind == LineKind::ReplaceInserted {
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

        let fmt = line.fmt();
        writeln!(f, "{}", fmt)?;
        Ok(())
    }
}

fn get_with_pos(line: &Line) -> Option<(usize, LineKind)> {
    match line.kind {
        LineKind::ReplaceRemoved => Some((line.old_pos?, line.kind)),
        LineKind::ReplaceInserted => Some((line.old_pos?, line.kind)),
        _ => None
    }
}

fn get_inverted(line: &Line) -> Option<(usize, LineKind)> {
    get_with_pos(line).map(|(pos, kind)| (pos, kind.invert()))
}

impl<'a> Display for Hunk<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lines = self
            .lines
            .iter()
            .filter(|line| line.kind.is_replaced())
            .filter_map(|line| {
                get_with_pos(line).map(|key| (key, (*line).clone()))
            })
            .collect::<BTreeMap<(usize, LineKind), Line>>();


        let header = format!(
            "... ...   @@ -{},{} +{},{} @@",
            self.old_start, self.removed, self.new_start, self.inserted
        );
        writeln!(f, "{}", header.on_blue().black().dimmed())?;

        for line in self.lines.iter() {
            if let Some(inverted) = get_inverted(&line).and_then(|key| lines.get(&key)) {
                LineDiff(inverted, line).fmt(f)?;
                continue;
            }

            writeln!(f, "{}", line.fmt())?;
        }
        Ok(())
    }
}
