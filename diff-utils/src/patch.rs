/*

Here is code for creating nice patch

*/
use crate::{CompareResult, Hunk};
use chrono::format::{DelayedFormat, StrftimeItems};
use std::borrow::Cow;
use std::fmt;

/// Options for creating patch files
#[derive(Clone, Copy, Debug)]
pub struct PatchOptions {
    /// Sometimes user want's to compare only subslice of a full str. This argument gives
    /// possibility to "move" whole patch to proper offset.
    ///
    /// # Example
    ///
    /// ```rust
    /// use diff_utils::{Comparison, PatchOptions};
    /// use chrono::{DateTime, Utc, TimeZone};
    ///
    /// let file_a = (0..1000).map(|i| if i%2 == 0 { "foo\n" } else { "bar\n" }).collect::<Vec<&str>>();
    /// let file_b = (0..1000).map(|i| if i%5 == 0 { "foo\n" } else { "bar\n" }).collect::<Vec<&str>>();
    ///
    /// let subslice_a = file_a.into_iter().skip(123).take(10).collect::<Vec<&str>>();
    /// let subslice_b = file_b.into_iter().skip(123).take(10).collect::<Vec<&str>>();
    ///
    /// let left_datetime: DateTime<Utc> = Utc.ymd(2015, 2, 18).and_hms(23, 16, 9);
    /// let left_dt = left_datetime.format("%F %T %z");
    /// let right_datetime: DateTime<Utc> = Utc.ymd(2020, 4, 20).and_hms(4, 20, 4);
    /// let right_dt = right_datetime.format("%F %T %z");
    ///
    /// let result = Comparison::new(&subslice_a, &subslice_b).compare().unwrap();
    /// println!("{}", result.patch(
    ///         "left.txt".into(),
    ///         &left_dt,
    ///         "right.txt".into(),
    ///         &right_dt,
    ///         PatchOptions { offset: 123, ..Default::default() }));
    /// ```
    ///
    /// Thanks to the `offset` the output will be:
    /// ```ignore
    /// --- left.txt    2015-02-18 23:16:09 +0000
    /// +++ right.txt    2020-04-20 04:20:04 +0000
    /// @@ -124,10 +124,10 @@
    ///  bar
    /// -foo
    ///  bar
    ///  foo
    ///  bar
    /// -foo
    ///  bar
    /// -foo
    /// +bar
    ///  bar
    ///  foo
    /// +bar
    /// +bar
    /// ```
    ///
    /// Default value: 1 - because in IT we count offsets from 0 but in files we count lines from 1
    pub offset: usize,
}

impl Default for PatchOptions {
    fn default() -> Self {
        Self {
            offset: 1
        }
    }
}

impl<'a> Hunk<'a> {
    /// Returns a structure which implements [`Display`](std::fmt::Display) for generating patch
    /// in [Unified Patch Format](https://www.gnu.org/software/diffutils/manual/html_node/Unified-Format.html).
    pub fn patch(&self, options: PatchOptions) -> HunkPatch {
        HunkPatch {
            hunk: self,
            options
        }
    }
}

/// Structure which implements [`Display`](std::fmt::Display) for generating patch in
/// in [Unified Patch Format](https://www.gnu.org/software/diffutils/manual/html_node/Unified-Format.html).
/// It is a wrapper to the [`Hunk`](struct.Hunk.html).
#[derive(Debug)]
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

impl<'a> CompareResult<'a> {
    /// Returns a structure which implements [`Display`](std::fmt::Display) for generating patch
    /// in [Unified Patch Format](https://www.gnu.org/software/diffutils/manual/html_node/Unified-Format.html).
    pub fn patch(
        &'a self,
        left_name: Cow<'a, str>,
        left_dt: &'a DelayedFormat<StrftimeItems<'a>>,
        right_name: Cow<'a, str>,
        right_dt: &'a DelayedFormat<StrftimeItems<'a>>,
        options: PatchOptions,
    ) -> CompareResultPatch<'a> {
        CompareResultPatch {
            result: self,
            left_name,
            right_name,
            left_dt,
            right_dt,
            options,
        }
    }
}

/// Structure which implements [`Display`](std::fmt::Display) for generating patch in
/// in [Unified Patch Format](https://www.gnu.org/software/diffutils/manual/html_node/Unified-Format.html).
/// It is a wrapper to the [`CompareResult`](struct.CompareResult.html).
#[derive(Debug)]
pub struct CompareResultPatch<'a> {
    result: &'a CompareResult<'a>,
    left_name: Cow<'a, str>,
    right_name: Cow<'a, str>,
    left_dt: &'a DelayedFormat<StrftimeItems<'a>>,
    right_dt: &'a DelayedFormat<StrftimeItems<'a>>,
    options: PatchOptions,
}

impl<'a> fmt::Display for CompareResultPatch<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- {}\t{}", self.left_name, self.left_dt)?;
        writeln!(f, "+++ {}\t{}", self.right_name, self.right_dt)?;
        for hunk in &self.result.hunks {
            hunk.patch(self.options).fmt(f)?;
        }
        Ok(())
    }
}
