#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! # About this crate
//! It is a hearth of our [`diff-assert`](../diff-assert/index.html) crate. It contains all logic
//! necessary to compare two text files and to produce diff or patch file.
//! It uses `diffs` library under the hood to perform patience algorithm.
//!
//! # Usage
//!
//! The most important structure is [`Comparison`](struct.Comparison.html). It gives you nice
//! interface for comparing string slices:
//!
//! ```rust
//! use diff_utils::Comparison;
//! let result = Comparison::new(&["foo", "bar"], &["foo", "foo"]).compare().expect("Comparison failed");
//! ```
//!
//! The result can be either printed (with `display` feature - see [`display`](struct.CompareResult#method.display) method),
//! used to generate patch (with `patch` feature - see [`patch`](struct.CompareResult#method.patch) method), or to simply
//! check if two files were equal:
//!
//! ```rust
//! # use diff_utils::Comparison;
//! # let result = Comparison::new(&["foo", "bar"], &["foo", "foo"]).compare().expect("Comparison failed");
//! if result.is_empty() {
//!     println!("Files have identical content");
//! }
//! ```
//!
//! # Features:
//! * `display` - to pretty print hunks in the console,
//! * `patch` to generate patch files

mod context;
mod hunk;
mod line;
mod processor;

#[cfg(feature = "display")]
mod display;

#[cfg(feature = "patch")]
mod patch;

use crate::context::Context;
use crate::processor::Processor;
use std::io;

pub use crate::hunk::Hunk;
pub use crate::line::{Line, LineKind};

#[cfg(feature = "display")]
pub use crate::display::DisplayOptions;

#[cfg(feature = "patch")]
pub use crate::patch::PatchOptions;

/// Main structure used to compare two slices of (in most cases) files.
/// It performs `Patience` diff algorithm.
///
/// # Example
/// ```rust
/// use diff_utils::Comparison;
/// let result = Comparison::new(&["foo", "bar"], &["foo", "foo"]).compare().expect("Comparison failed");
/// ```
#[derive(Debug)]
pub struct Comparison<'a> {
    /// Left/old file slice
    pub left: &'a [&'a str],
    /// Right/new file slice
    pub right: &'a [&'a str],
    /// Context radius. Number of equal lines attached to each hunk before and after. Default: 3
    pub context_radius: usize,
}

impl<'a> Comparison<'a> {
    /// Constructor. Both slices should represent sequences of lines.
    pub fn new(left: &'a [&'a str], right: &'a [&'a str]) -> Self {
        Self {
            left,
            right,
            context_radius: 3,
        }
    }

    /// Perform comparision
    ///
    /// # Errors
    /// In case of any errors in patience algorithm it may return `io::Error`.
    pub fn compare(&self) -> io::Result<CompareResult<'a>> {
        let mut processor = Processor::new(&self.left, &self.right, self.context_radius);
        {
            let mut replace = diffs::Replace::new(&mut processor);
            diffs::patience::diff(
                &mut replace,
                self.left,
                0,
                self.left.len(),
                self.right,
                0,
                self.right.len(),
            )?;
        }
        Ok(CompareResult {
            hunks: processor.result(),
        })
    }
}

/// The actual result of a comparison. It contains the list of the hunks with line differences.
#[derive(Debug)]
pub struct CompareResult<'a> {
    pub(crate) hunks: Vec<Hunk<'a>>,
}

impl<'a> CompareResult<'a> {
    /// If the comparsion finds no differences, it returns `true`.
    pub fn is_empty(&self) -> bool {
        self.hunks.is_empty()
    }

    /// Slice of the sequence of hunks.
    pub fn hunks(&self) -> &[Hunk<'a>] {
        &self.hunks
    }
}

/// Performs diff and returns list of hunks.
/// # Breaking change
/// it requires `&'a str` instead of `&'a String`.
#[deprecated(
    since = "0.3.0",
    note = "Instead you should use `Comparison::new(..).compare(..)`"
)]
pub fn diff_hunks<'a>(
    left: &'a [&'a str],
    right: &'a [&'a str],
    context_radius: usize,
) -> std::io::Result<Vec<Hunk<'a>>> {
    let comparison = Comparison {
        left,
        right,
        context_radius,
    }
    .compare()?;

    Ok(comparison.hunks)
}

/// Performs diff on two files and returns formatted display.
#[cfg(feature = "display")]
#[deprecated(
    since = "0.3.0",
    note = "Instead you should use `Comparison::new(..).compare(..)`"
)]
#[allow(deprecated)]
pub fn diff(
    text1: &[String],
    text2: &[String],
    context_radius: usize,
) -> std::io::Result<Vec<String>> {
    let left = text1.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
    let right = text2.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();

    let result = diff_hunks(&left, &right, context_radius)?
        .into_iter()
        .map(|hunk| format!("{}", hunk.display(Default::default())))
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use test_case::test_case;

    #[test_case("A B C\nD E F", "A B D\nE F G")]
    #[test_case("A B C\nD E F", "A B D\nE F G\n1 2 3")]
    #[test_case(
    r#"|-------|-------------|-----------------------------------------|-----|---------------------------------|-----------|
| 24638 | Twin Room   | P1:3 A1:3 C0:2 FC0:9[0:1] MCA0:3[13:13] | DZ  | child_ages:["2:3","4:6","7:12"] |           |"#,
    r#"|-------|-------------|-----------------------------------------|-----|---------------------------|-----------|
| 24638 | Twin Room   | P1:3 A1:3 C0:2 FC0:9[0:3] MCA0:3[13:13] | DZ  | child_ages:["4:6","7:12"] |           |"#
    )]
    fn test_diff_hunks(left: &str, right: &str) {
        let left: Vec<&str> = left.lines().collect();
        let right: Vec<&str> = right.lines().collect();
        let result = Comparison::new(&left, &right).compare().expect("hunks");

        if !result.is_empty() {
            let hunks = result
                .hunks
                .iter()
                .map(|s| format!("{}\n", s.display(Default::default())))
                .join("\n");
            insta::assert_display_snapshot!(hunks);
        }
    }

    #[test_case(
    "\n\u{1b}[1;4mLorem ipsum\u{1b}[0m\n\n\nExcepteur sint occaecat cupidatat non proident\n\n\u{1b}[7m1\u{1b}[0m\n",
    "\n\u{1b}[1;4mLorem ipsum\u{1b}[0m\n\n\nExcepteur sint occaecat cupidatat non proident\n\n\u{1b}[7m2\u{1b}[0m\n"
    )]
    #[test_case(
        "\nLorem ipsum\n\n\nExcepteur sint occaecat cupidatat non proident\n\n1\n",
        "\nLorem ipsum\n\n\nExcepteur sint occaecat cupidatat non proident\n\n2\n"
    )]
    #[test_case("\nL\n\n\nE\n\n1\n", "\nL\n\n\nE\n\n2\n")]
    #[test_case("\n\n\n\n\n\n1", "\n\n\n\n\n\n2")]
    fn overflow_test(a: &str, b: &str) {
        let left: Vec<&str> = a.lines().collect();
        let right: Vec<&str> = b.lines().collect();
        let hunks = Comparison::new(&left, &right).compare().expect("hunks");
        dbg!(&hunks);
    }

    #[test_case(
        "\nLorem \n\n\nipsum\n1\n2\n3\n4\n",
        "\nLorem \n\n\nipsun\n1\n2\n3\n4\n"
    )]
    #[test_case("\nLorem \n\n\nipsum\n1\n2\n3\n", "\nLorem \n\n\nipsun\n1\n2\n3\n")]
    #[test_case("\nLorem \n\n\nipsum\n1\n", "\nLorem \n\n\nipsun\n1\n")]
    #[test_case(
    concat!(
    "1\n2\n3\n4\n", // unchanged
    "foo\n", // changed
    "1\n2\n3\n4\n", // unchanged
    "bar\n", // changed
    "1\n2\n3\n4\n", // unchanged
    ),
    concat!(
    "1\n2\n3\n4\n", // unchanged
    "fou\n", // changed
    "1\n2\n3\n4\n", // unchanged
    "baz\n", // changed
    "1\n2\n3\n4\n", // unchanged
    )
    )]
    fn bad_diff_test(a: &str, b: &str) {
        let left: Vec<&str> = a.lines().collect();
        let right: Vec<&str> = b.lines().collect();
        let hunks = Comparison::new(&left, &right).compare().expect("hunks");
        insta::assert_debug_snapshot!(hunks);
    }
}
