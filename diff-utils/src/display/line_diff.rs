use crate::{Comparison, DisplayOptions, Line, LineKind};
use colored::Colorize;
use itertools::Itertools;
use std::fmt;

pub(crate) struct LineDiff<'a> {
    pub(crate) left: &'a Line<'a>,
    pub(crate) right: &'a Line<'a>,
    pub(crate) options: DisplayOptions<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_support() {
        colored::control::set_override(false);

        let left = "Pośród";
        let right = "Posród"; // "s" instead of "ś".

        let left = Line::replace_remove(1, Some(2), left);
        let right = Line::replace_insert(Some(1), 2, right);

        let diff = LineDiff {
            left: &left,
            right: &right,
            options: Default::default(),
        };

        assert_eq!("    003  +Posród\n", diff.to_string());
    }
}

impl<'a> fmt::Display for LineDiff<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self
            .left
            .inner
            .char_indices()
            .map(|(idx, c)| &self.left.inner[idx..idx + c.len_utf8()])
            .collect::<Vec<_>>();
        let r = self
            .right
            .inner
            .char_indices()
            .map(|(idx, c)| &self.right.inner[idx..idx + c.len_utf8()])
            .collect::<Vec<_>>();

        let len = std::cmp::max(self.left.inner.len(), self.right.inner.len());
        let diff = Comparison {
            left: &l,
            right: &r,
            context_radius: len,
        }
        .compare()
        .unwrap();
        if diff.is_empty() {
            return writeln!(f, "{}", self.right.display(self.options));
        }
        let hunk = &diff.hunks[0];

        let line = hunk
            .lines
            .iter()
            .filter(|l| l.kind != LineKind::Removed && l.kind != LineKind::ReplaceRemoved)
            .map(|letter| {
                if letter.kind == LineKind::Unchanged {
                    format!("{}", letter.inner.dimmed())
                } else if letter.kind == LineKind::Inserted
                    || letter.kind == LineKind::ReplaceInserted
                {
                    format!("{}", letter.inner.reversed())
                } else {
                    unreachable!("Filters removed. Can't happen")
                }
            })
            .join("");

        let line = Line {
            inner: &line,
            ..self.right.clone()
        };

        let fmt = line.display(self.options);
        writeln!(f, "{}", fmt)?;
        Ok(())
    }
}
