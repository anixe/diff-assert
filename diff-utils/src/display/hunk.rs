use crate::display::line_diff::LineDiff;
use crate::{DisplayOptions, Hunk, Line, LineKind};
use colored::Colorize;
use std::collections::BTreeMap;
use std::fmt;

impl<'a> Hunk<'a> {
    /// Returns a structure which implements [`Display`](std::fmt::Display) with ANSI escape color codes.
    pub fn display(&'a self, options: DisplayOptions<'a>) -> HunkDisplay<'a> {
        HunkDisplay {
            hunk: self,
            options,
        }
    }
}

/// Structure which implements [`Display`](std::fmt::Display) with ANSI escape color codes. It is a
/// wrapper to the [`Hunk`](struct.Hunk.html).
#[derive(Debug)]
pub struct HunkDisplay<'a> {
    hunk: &'a Hunk<'a>,
    options: DisplayOptions<'a>,
}

impl<'a> fmt::Display for HunkDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lines = self
            .hunk
            .lines
            .iter()
            .filter(|line| line.kind.is_replaced())
            .filter_map(|line| get_with_pos(line).map(|key| (key, (*line).clone())))
            .collect::<BTreeMap<(usize, LineKind), Line>>();

        let header = format!(
            "... ...   @@ -{},{} +{},{} @@",
            self.hunk.old_start + self.options.offset,
            self.hunk.removed,
            self.hunk.new_start + self.options.offset,
            self.hunk.inserted
        );
        writeln!(f, "{}", header.black().dimmed())?;

        for line in self.hunk.lines.iter() {
            if let Some(inverted) = get_inverted(line).and_then(|key| lines.get(&key)) {
                LineDiff {
                    left: inverted,
                    right: line,
                    options: self.options,
                }
                .fmt(f)?;
                continue;
            }

            writeln!(f, "{}", line.display(self.options))?;
        }
        Ok(())
    }
}

fn get_with_pos(line: &Line) -> Option<(usize, LineKind)> {
    match line.kind {
        LineKind::ReplaceRemoved => Some((line.old_pos?, line.kind)),
        LineKind::ReplaceInserted => Some((line.old_pos?, line.kind)),
        _ => None,
    }
}

fn get_inverted(line: &Line) -> Option<(usize, LineKind)> {
    get_with_pos(line).map(|(pos, kind)| (pos, kind.invert()))
}
