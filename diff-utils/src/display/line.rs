use crate::display::DisplayOptions;
use crate::{Line, LineKind};
use colored::Colorize;
use std::fmt;

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
