use crate::{CompareResult, DisplayOptions};
use itertools::Itertools;
use std::fmt;

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
