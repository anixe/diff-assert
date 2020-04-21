/*

Here is code for creating nice patch

*/
use crate::{Comparison, Hunk};
use chrono::format::{DelayedFormat, StrftimeItems};
use std::borrow::Cow;
use std::fmt;

#[derive(Clone, Copy, Default)]
pub struct PatchOptions {
    pub offset: usize,
}

impl<'a> Hunk<'a> {
    pub fn patch(&self, options: PatchOptions) -> HunkPatch {
        HunkPatch {
            hunk: self,
            options,
        }
    }
}

pub struct HunkPatch<'a> {
    hunk: &'a Hunk<'a>,
    options: PatchOptions,
}

impl<'a> fmt::Display for HunkPatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = format!(
            "@@ -{},{} +{},{} @@",
            self.hunk.old_start + self.options.offset,
            self.hunk.removed,
            self.hunk.new_start + self.options.offset,
            self.hunk.inserted,
        );
        writeln!(f, "{}", header)?;

        for line in self.hunk.lines.iter() {
            let sign = line.kind.sign();
            writeln!(f, "{}{}", sign, line.inner)?;
        }
        Ok(())
    }
}

impl<'a> Comparison<'a> {
    pub fn patch(
        &self,
        left_name: Cow<'a, str>,
        left_dt: &'a DelayedFormat<StrftimeItems<'a>>,
        right_name: Cow<'a, str>,
        right_dt: &'a DelayedFormat<StrftimeItems<'a>>,
        options: PatchOptions,
    ) -> ComparisonPatch {
        ComparisonPatch {
            comparison: self,
            left_name,
            left_dt,
            right_name,
            right_dt,
            options,
        }
    }
}

pub struct ComparisonPatch<'a> {
    comparison: &'a Comparison<'a>,
    left_name: Cow<'a, str>,
    right_name: Cow<'a, str>,
    left_dt: &'a DelayedFormat<StrftimeItems<'a>>,
    right_dt: &'a DelayedFormat<StrftimeItems<'a>>,
    options: PatchOptions,
}

impl<'a> fmt::Display for ComparisonPatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- {}\t{}", self.left_name, self.left_dt)?;
        writeln!(f, "+++ {}\t{}", self.right_name, self.right_dt)?;
        for hunk in self.comparison.compare().unwrap().hunks {
            hunk.patch(self.options).fmt(f)?;
        }
        Ok(())
    }
}
