#![deny(
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
//! We designed this crate to replace `assert_eq!` with more powerful `assert_diff` which can show
//! exactly what part of actual data differs from expected behaviour.
//!
//! You may wonder why another crate like this. We noticed that other crates have significant issues
//! with for example whitespaces and newlines, which brings us to this solution. It's well tested,
//! used in dozens of thousands cases in our ecosystem, and we believe it is simply more correct.
//!
//! # How to use it?
//!
//! The simplest example is to use `assert_diff!` macro:
//! ```rust,should_panic
//! # #[macro_use] extern crate diff_assert;
//! let expected = r#"foo
//! bar"#;
//!
//! let actual = r#"foo
//! foo"#;
//!
//! assert_diff!(expected, actual, "Here is an optional message what has changed");
//! ```

pub use diff_utils::*;
use std::str::Lines;

/// Asserts equality between `Debug` output of any two objects.
/// Internally it uses `try_dbg!` and then panics if outputs are not equal.
///
/// # Input
/// `$expected` - Expected outcome. Has to implement `Debug` trait,
/// `$actual` - Actual outcome. Has to implement `Debug` trait,
/// `$message_args` - Optional message when assertion fails.
///
/// # Panics
/// If expected != actual
///
/// # Examples
///
/// ```rust,should_panic
/// # #[macro_use] extern crate diff_assert;
/// # fn main() {
/// let expected = ("foo", "bar");
///
/// let actual = ("foo", "foo");
///
/// assert_dbg!(expected, actual, "Here is an optional message what has changed");
/// # }
/// ```
#[macro_export]
macro_rules! assert_dbg {
    ($expected: expr, $actual: expr) => {
        $crate::assert_diff!(
            format!("{:#?}", $expected),
            format!("{:#?}", $actual))
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::assert_diff!(
            format!("{:#?}", $expected),
            format!("{:#?}", $actual),
            $message $(,$message_args)*)
    }
}

/// Checks equality between `Debug` output of any two objects and returns Err(String) if it fails.
///
/// # Input
/// `$expected` - Expected outcome. Has to implement `Debug` trait,
/// `$actual` - Actual outcome. Has to implement `Debug` trait,
/// `$message_args` - Optional message when objects are not equal.
///
/// # Errors
/// When `$expected` != `$actual`
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate diff_assert;
/// # fn main() {
/// let expected = ("foo", "bar");
///
/// let actual = ("foo", "foo");
///
/// if let Err(e) = try_dbg!(expected, actual, "Here is an optional message what has changed") {
///     eprintln!("{}", e);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! try_dbg {
    ($expected: expr, $actual: expr) => {
        $crate::try_diff!(
            format!("{:#?}", $expected),
            format!("{:#?}", $actual))
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::try_diff!(
            format!("{:#?}", $expected),
            format!("{:#?}", $actual),
            $message $(,$message_args)*)
    }
}

/// Checks equality between output of any two objects and returns Err(String) if it fails.
/// This macro requires that arguments have method:
/// ```ignore
/// fn lines(&self) -> std::str::Lines;
/// ```
///
/// # Input
/// `$expected` - Expected outcome,
/// `$actual` - Actual outcome,
/// `$message_args` - Optional message when objects are not equal.
///
/// # Errors
/// When `$expected` != `$actual`
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate diff_assert;
/// # fn main() {
/// let expected = r#"foo
/// bar"#;
///
/// let actual = r#"foo
/// foo"#;
///
/// if let Err(e) = try_diff!(expected, actual, "Here is an optional message what has changed") {
///     eprintln!("{}", e);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! try_diff {
    ($expected: expr, $actual: expr) => {
        $crate::try_diff!($expected, $actual, "Found differences")
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::inner_try_diff($expected.lines(), $actual.lines(), format!($message, $($message_args),*))
    };
}

/// Asserts equality between lines of any two objects.
/// Internally it uses `try_diff!` and then panics if outputs are not equal.
/// This macro requires that arguments have method:
/// ```ignore
/// fn lines(&self) -> std::str::Lines;
/// ```
///
/// # Input
/// `$expected` - Expected outcome,
/// `$actual` - Actual outcome,
/// `$message_args` - Optional message when assertion fails.
///
/// # Panics
/// If expected != actual
///
/// # Examples
///
/// ```rust,should_panic
/// # #[macro_use] extern crate diff_assert;
/// # fn main() {
/// let expected = r#"foo
/// bar"#;
///
/// let actual = r#"foo
/// foo"#;
///
/// assert_diff!(expected, actual, "Here is an optional message what has changed");
/// # }
/// ```
#[macro_export]
macro_rules! assert_diff {
    ($expected: expr, $actual: expr) => {
        $crate::assert_diff!($expected, $actual, "Found differences")
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::inner_assert_diff($expected.lines(), $actual.lines(), format!($message, $($message_args),*))
    };
}

#[doc(hidden)]
pub fn inner_try_diff(expected: Lines, actual: Lines, msg_fmt: String) -> Result<(), String> {
    let e: Vec<&str> = expected.collect();
    let a: Vec<&str> = actual.collect();
    let result = Comparison::new(&e, &a).compare().unwrap();
    if !result.is_empty() {
        Err(result
            .display(DisplayOptions {
                offset: 0,
                msg_fmt: &msg_fmt,
            })
            .to_string())
    } else {
        Ok(())
    }
}

#[doc(hidden)]
pub fn inner_assert_diff(expected: Lines, actual: Lines, msg_fmt: String) {
    if let Err(e) = inner_try_diff(expected, actual, msg_fmt) {
        panic!("{}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test() {
        let expected = "foo
        bar"
        .to_string();

        let actual = "foo
        foo"
        .to_string();

        assert_diff!(expected, actual);
    }

    #[test]
    fn try_test() {
        let expected = "foo
        bar"
        .to_string();

        let actual = "foo
        foo"
        .to_string();

        assert!(try_diff!(expected, actual).is_err());
    }

    #[test]
    #[should_panic]
    fn dbg_test() {
        let expected = ("Foo", "Bar");
        let actual = ("Foo", "foo");
        assert_dbg!(expected, actual);
    }
}
