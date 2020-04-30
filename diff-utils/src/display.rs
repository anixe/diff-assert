/*

Here is code for displaying nice diff

*/
use crate::{CompareResult, Comparison, Hunk, Line, LineKind};
use colored::*;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt;

/// Options for displaying diffs.
#[derive(Clone, Copy, Default, Debug)]
pub struct DisplayOptions<'a> {
    /// Sometimes user want's to compare only subslice of a full str. This argument gives
    /// possibility to "move" whole diff to proper offset.
    ///
    /// # Example
    ///
    /// ```rust
    /// use diff_utils::{Comparison, DisplayOptions};
    /// let file_a = (0..1000).map(|i| if i%2 == 0 { "foo\n" } else { "bar\n" }).collect::<Vec<&str>>();
    /// let file_b = (0..1000).map(|i| if i%5 == 0 { "foo\n" } else { "bar\n" }).collect::<Vec<&str>>();
    ///
    /// let subslice_a = file_a.into_iter().skip(123).take(10).collect::<Vec<&str>>();
    /// let subslice_b = file_b.into_iter().skip(123).take(10).collect::<Vec<&str>>();
    ///
    /// let result = Comparison::new(&subslice_a, &subslice_b).compare().unwrap();
    /// println!("{}", result.display(DisplayOptions { offset: 123, ..Default::default() }));
    /// ```
    ///
    /// Thanks to the `offset` the output will be:
    /// ```ignore
    /// ... ...   @@ -124,10 +124,10 @@
    /// 124 124   bar
    /// 125      -foo
    /// 126 125   bar
    /// 127 126   foo
    /// 128 127   bar
    /// 129      -foo
    /// 130 128   bar
    /// 131      -foo
    ///     129  +bar
    /// 132 130   bar
    /// 133 131   foo
    ///     132  +bar
    ///     133  +bar
    /// ```
    ///
    /// Default value: 0
    pub offset: usize,
    /// Print extra message before writing diff itself.
    /// It is mostly used to specify the filenames
    pub msg_fmt: &'a str,
}

impl<'a> Line<'a> {
    /// Returns a structure which implements [`Display`](std::fmt::Display) with ANSI escape color codes.
    pub fn display(&'a self, options: DisplayOptions<'a>) -> LineDisplay<'a> {
        LineDisplay {
            line: self,
            options,
        }
    }
}

/// Structure which implements [`Display`](std::fmt::Display) with ANSI escape color codes. It is a
/// wrapper to the [`Line`](struct.Line.html).
#[derive(Debug)]
pub struct LineDisplay<'a> {
    line: &'a Line<'a>,
    options: DisplayOptions<'a>,
}

impl<'a> fmt::Display for LineDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = self.line.inner;
        let i = self.line.old_pos.map(|p| p + 1 + self.options.offset);
        let j = self.line.new_pos.map(|p| p + 1 + self.options.offset);
        let sign = self.line.kind.sign();

        let header = match self.line.kind {
            LineKind::Inserted | LineKind::ReplaceInserted => {
                format!("    {:03}  {}", j.unwrap(), sign.bold())
            }
            LineKind::Removed | LineKind::ReplaceRemoved => {
                format!("{:03}      {}", i.unwrap(), sign.bold())
            }
            LineKind::Unchanged => format!("{:03} {:03}   ", i.unwrap(), j.unwrap()),
        };

        match self.line.kind {
            LineKind::Inserted | LineKind::ReplaceInserted => {
                write!(f, "{}", header.on_black().green())
            }
            LineKind::Removed | LineKind::ReplaceRemoved => {
                write!(f, "{}", header.on_black().red())
            }
            LineKind::Unchanged => write!(f, "{}", header),
        }?;

        match self.line.kind {
            LineKind::ReplaceInserted => write!(f, "{}", line.on_black().green()),
            LineKind::ReplaceRemoved => write!(f, "{}", line.on_black().red()),
            LineKind::Inserted => write!(f, "{}", line.on_green().black()),
            LineKind::Removed => write!(f, "{}", line.on_red().black()),
            LineKind::Unchanged => write!(f, "{}", line),
        }
    }
}

struct LineDiff<'a> {
    left: &'a Line<'a>,
    right: &'a Line<'a>,
    options: DisplayOptions<'a>,
}

impl<'a> fmt::Display for LineDiff<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self
            .left
            .inner
            .char_indices()
            .map(|(idx, _)| &self.left.inner[idx..=idx])
            .collect::<Vec<_>>();
        let r = self
            .right
            .inner
            .char_indices()
            .map(|(idx, _)| &self.right.inner[idx..=idx])
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
        writeln!(f, "{}", header.on_blue().black().dimmed())?;

        for line in self.hunk.lines.iter() {
            if let Some(inverted) = get_inverted(&line).and_then(|key| lines.get(&key)) {
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

impl<'a> CompareResult<'a> {
    /// Returns a structure which implements [`Display`](std::fmt::Display) with ANSI escape color codes.
    pub fn display(&'a self, options: DisplayOptions<'a>) -> CompareResultDisplay<'a> {
        CompareResultDisplay {
            result: self,
            options,
        }
    }
}

/// Structure which implements [`Display`](std::fmt::Display) with ANSI escape color codes. It is a
/// wrapper to the [`CompareResult`](struct.CompareResult.html).
#[derive(Debug)]
pub struct CompareResultDisplay<'a> {
    result: &'a CompareResult<'a>,
    options: DisplayOptions<'a>,
}

impl<'a> fmt::Display for CompareResultDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.result.is_empty() {
            let mut msg = String::from("\n");
            msg += self.options.msg_fmt;
            msg += "\n\n";

            msg += &self
                .result
                .hunks
                .iter()
                .map(|s| s.display(self.options).to_string())
                .join("\n");

            write!(f, "{}", msg)
        } else {
            Ok(())
        }
    }
}
