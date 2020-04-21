mod context;
mod hunk;
mod line;
mod processor;

#[cfg(feature = "display")]
mod display;

#[cfg(feature = "patch")]
mod patch;

pub use crate::context::Context;
pub use crate::hunk::Hunk;
pub use crate::line::{Line, LineKind};
pub use crate::processor::Processor;

#[cfg(feature = "display")]
pub use crate::display::DisplayOptions;

#[cfg(feature = "patch")]
pub use crate::patch::PatchOptions;

use std::io;

pub struct Comparison<'a> {
    pub left: &'a [&'a str],
    pub right: &'a [&'a str],
    pub context_radius: usize,
}

impl<'a> Comparison<'a> {
    pub fn new(left: &'a [&'a str], right: &'a [&'a str]) -> Self {
        Self {
            left,
            right,
            context_radius: 3,
        }
    }

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

#[derive(Debug)]
pub struct CompareResult<'a> {
    pub hunks: Vec<Hunk<'a>>,
}

impl<'a> CompareResult<'a> {
    pub fn is_empty(&self) -> bool {
        self.hunks.is_empty()
    }
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
