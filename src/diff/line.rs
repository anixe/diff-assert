use colored::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Line<'a> {
    pub(crate) kind: LineKind,
    pub(crate) inner: &'a str,
    pub(crate) old_pos: usize,
    pub(crate) new_pos: usize,
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Copy)]
pub enum LineKind {
    Removed,
    Inserted,
    Unchanged,
}

impl LineKind {
    pub fn invert(&self) -> Self {
        match self {
            LineKind::Removed => LineKind::Inserted,
            LineKind::Inserted => LineKind::Removed,
            u => *u,
        }
    }
}

impl<'a> Line<'a> {
    pub fn insert(pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Inserted,
            inner,
            old_pos: pos,
            new_pos: pos,
        }
    }

    pub fn remove(pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Removed,
            inner,
            old_pos: pos,
            new_pos: pos,
        }
    }

    pub fn line(old_pos: usize, new_pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Unchanged,
            inner,
            old_pos,
            new_pos,
        }
    }

    pub fn fmt<'b>(&'b self) -> LineFmt<'b> {
        LineFmt(self, false)
    }
}

pub struct LineFmt<'a>(pub &'a Line<'a>, pub bool); // line, reversed

impl<'a> Display for LineFmt<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let line = self.0.inner;
        let i = self.0.old_pos + 1;
        let j = self.0.new_pos + 1;
        let sign = match self.0.kind {
            LineKind::Inserted => "+",
            LineKind::Removed => "-",
            LineKind::Unchanged => " ",
        };

        let header = match self.0.kind {
            LineKind::Inserted => format!("    {:03}  {}", j, sign.bold()),
            LineKind::Removed => format!("{:03}      {}", i, sign.bold()),
            LineKind::Unchanged => format!("{:03} {:03}   ", i, j),
        };

        match self.0.kind {
            LineKind::Inserted => write!(f, "{}", header.on_black().green()),
            LineKind::Removed => write!(f, "{}", header.on_black().red()),
            LineKind::Unchanged => write!(f, "{}", header),
        }?;

        match self.0.kind {
            LineKind::Inserted if self.1 => write!(f, "{}", line.on_black().green()),
            LineKind::Removed if self.1 => write!(f, "{}", line.on_black().red()),
            LineKind::Inserted => write!(f, "{}", line.on_green().black()),
            LineKind::Removed => write!(f, "{}", line.on_red().black()),
            LineKind::Unchanged => write!(f, "{}", line),
        }
    }
}
