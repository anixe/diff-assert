use colored::*;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Line<'a> {
    pub(crate) kind: LineKind,
    pub(crate) inner: &'a str,
    pub(crate) old_pos: Option<usize>,
    pub(crate) new_pos: Option<usize>,
}

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Copy)]
pub enum LineKind {
    Removed,
    Inserted,
    ReplaceRemoved,
    ReplaceInserted,
    Unchanged,
}

impl LineKind {
    pub fn invert(&self) -> Self {
        match self {
            LineKind::Removed => LineKind::Inserted,
            LineKind::Inserted => LineKind::Removed,
            LineKind::ReplaceInserted => LineKind::ReplaceRemoved,
            LineKind::ReplaceRemoved => LineKind::ReplaceInserted,
            u => *u,
        }
    }

    pub fn sign(&self) -> &str {
        match self {
            LineKind::ReplaceInserted | LineKind::Inserted => "+",
            LineKind::ReplaceRemoved | LineKind::Removed => "-",
            LineKind::Unchanged => " ",
        }
    }

    pub fn is_replaced(&self) -> bool {
        match self {
            LineKind::ReplaceInserted | LineKind::ReplaceRemoved => true,
            _ => false
        }
    }
}

impl<'a> Line<'a> {
    pub fn insert(pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Inserted,
            inner,
            old_pos: None,
            new_pos: Some(pos),
        }
    }

    pub fn remove(pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Removed,
            inner,
            old_pos: Some(pos),
            new_pos: None,
        }
    }

    pub fn replace_insert(old_pos: Option<usize>, new_pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::ReplaceInserted,
            inner,
            old_pos,
            new_pos: Some(new_pos),
        }
    }

    pub fn replace_remove(old_pos: usize, new_pos: Option<usize>, inner: &'a str) -> Self {
        Line {
            kind: LineKind::ReplaceRemoved,
            inner,
            old_pos: Some(old_pos),
            new_pos,
        }
    }

    pub fn line(old_pos: usize, new_pos: usize, inner: &'a str) -> Self {
        Line {
            kind: LineKind::Unchanged,
            inner,
            old_pos: Some(old_pos),
            new_pos: Some(new_pos),
        }
    }

    pub fn fmt(&self) -> LineFmt {
        LineFmt(self)
    }
}

pub struct LineFmt<'a>(pub &'a Line<'a>); // line

impl<'a> Display for LineFmt<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let line = self.0.inner;
        let i = self.0.old_pos.map(|p| p + 1);
        let j = self.0.new_pos.map(|p| p + 1);
        let sign = self.0.kind.sign();

        let header = match self.0.kind {
            LineKind::Inserted | LineKind::ReplaceInserted => format!("    {:03}  {}", j.unwrap(), sign.bold()),
            LineKind::Removed | LineKind::ReplaceRemoved => format!("{:03}      {}", i.unwrap(), sign.bold()),
            LineKind::Unchanged => format!("{:03} {:03}   ", i.unwrap(), j.unwrap()),
        };

        match self.0.kind {
            LineKind::Inserted | LineKind::ReplaceInserted => write!(f, "{}", header.on_black().green()),
            LineKind::Removed | LineKind::ReplaceRemoved => write!(f, "{}", header.on_black().red()),
            LineKind::Unchanged => write!(f, "{}", header),
        }?;

        match self.0.kind {
            LineKind::ReplaceInserted => write!(f, "{}", line.on_black().green()),
            LineKind::ReplaceRemoved => write!(f, "{}", line.on_black().red()),
            LineKind::Inserted => write!(f, "{}", line.on_green().black()),
            LineKind::Removed => write!(f, "{}", line.on_red().black()),
            LineKind::Unchanged => write!(f, "{}", line),
        }
    }
}
