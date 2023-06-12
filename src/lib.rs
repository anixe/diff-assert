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
//! We designed this crate to replace `assert_eq!` with more powerful `assert_diff!` which can show
//! exactly what part of actual data differs from expected behaviour.
//!
//! You may wonder why another crate like this. We noticed that other crates have significant issues
//! with for example whitespaces and newlines, which brings us to this solution. It's well tested,
//! used in dozens of thousands cases in our ecosystem, and we believe it is simply more correct.
//!
//! # How to use it?
//!
//! The simplest example is to use [`assert_diff!`](macro.assert_diff.html) macro:
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
//!
//! Another possibility is to use [`try_diff!`](macro.try_diff.html) macro if you don't want to panic.
//! It returns nice `Result<(), String>` instead.
//! ```rust
//! # #[macro_use] extern crate diff_assert;
//! let expected = r#"foo
//! bar"#;
//!
//! let actual = r#"foo
//! foo"#;
//!
//! if let Err(e) = try_diff!(expected, actual, "Here is an optional message what has changed") {
//!     eprintln!("Oh nay, we got a difference!");
//!     eprintln!("{}", e);
//! }
//! ```
//!
//! Besides that, crate also contains similar macros for comparing not string slices but
//! [`Debug`](std::fmt::Debug) format outputs. It is quite handy for testing intermediate outputs.
//! * [`assert_dbg!`](macro.assert_dbg.html)
//! * [`try_dbg!`](macro.try_dbg.html)

pub use diff_utils::*;
use std::path::Path;
use std::str::Lines;

/// Asserts equality between [`Debug`](std::fmt::Debug) output of any two objects.
/// Internally it uses `try_dbg!` and then panics if outputs are not equal.
///
/// # Input
/// `$expected` - Expected outcome. Has to implement [`Debug`](std::fmt::Debug) trait,
/// `$actual` - Actual outcome. Has to implement [`Debug`](std::fmt::Debug) trait,
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

/// Checks equality between [`Debug`](std::fmt::Debug) output of any two objects and returns Err(String) if it fails.
///
/// # Input
/// `$expected` - Expected outcome. Has to implement [`Debug`](std::fmt::Debug) trait,
/// `$actual` - Actual outcome. Has to implement [`Debug`](std::fmt::Debug) trait,
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
/// let err = try_dbg!(expected, actual, "Here is an optional message what has changed").unwrap_err();
///
/// assert!(err.starts_with("\nHere is an optional message what has changed"));
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
/// let err = try_diff!(expected, actual, "Here is an optional message what has changed").unwrap_err();
///
/// assert!(err.starts_with("\nHere is an optional message what has changed"));
/// # }
/// ```
#[macro_export]
macro_rules! try_diff {
    ($expected: expr, $actual: expr) => {
        $crate::try_diff!($expected, $actual, "Found differences")
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::inner_try_diff($expected.lines(), $actual.lines(), &format!($message, $($message_args),*))
    };
}

/// Asserts equality between lines of any two objects.
/// Internally it uses [`try_diff!`](macro.try_diff.html) and then panics if outputs are not equal.
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
        $crate::inner_assert_diff($expected.lines(), $actual.lines(), &format!($message, $($message_args),*))
    };
}

/// Asserts equality between two files.
/// Internally it uses [`try_diff_file!`](macro.try_diff_file.html) and then panics if files are not equal.
/// This macro requires that arguments implement trait `AsRef<Path>`
///
/// # Input
/// `$expected` - path to the file with expected content,
/// `$actual` - path to the file with actual content,
/// `$message_args` - optional message when assertion fails.
///
/// # Panics
/// If expected file content != actual file content
///
/// # Examples
///
/// ```rust,should_panic
/// # #[macro_use] extern crate diff_assert;
/// # fn main() {
/// let expected = "tests/data/diff_file/different/a.txt";
/// let actual = "tests/data/diff_file/different/b.txt";
///
/// assert_diff_file!(expected, actual, "Here is an optional message what has changed");
/// # }
/// ```
#[macro_export]
macro_rules! assert_diff_file {
    ($expected: expr, $actual: expr) => { {
        let expected: &::std::path::Path = $expected.as_ref();
        let actual: &::std::path::Path = $actual.as_ref();
        $crate::assert_diff_file!(expected, actual, "Found differences between {} and {}", expected.display(), actual.display())
    } };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::inner_assert_diff_file($expected, $actual, &format!($message, $($message_args),*))
    };
}

/// Checks equality between two files and returns Err(String) if it fails.
/// This macro requires that arguments implement trait `AsRef<Path>`
///
/// # Input
/// `$expected` - path to the file with expected content,
/// `$actual` - path to the file with actual content,
/// `$message_args` - optional message when assertion fails.
///
/// # Panics
/// If expected file content != actual file content
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate diff_assert;
/// # fn main() {
/// let expected = "tests/data/diff_file/different/a.txt";
/// let actual = "tests/data/diff_file/different/b.txt";
///
/// let err = try_diff_file!(expected, actual, "Here is an optional message what has changed").unwrap_err();
///
/// assert!(err.starts_with("\nHere is an optional message what has changed"));
/// # }
/// ```
#[macro_export]
macro_rules! try_diff_file {
    ($expected: expr, $actual: expr) => { {
        let expected: &::std::path::Path = $expected.as_ref();
        let actual: &::std::path::Path = $actual.as_ref();
        $crate::try_diff_file!(expected, actual, "Found differences between {} and {}", expected.display(), actual.display())
    } };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::inner_try_diff_file($expected, $actual, &format!($message, $($message_args),*))
    };
}

#[doc(hidden)]
pub fn inner_try_diff(expected: Lines, actual: Lines, msg_fmt: &str) -> Result<(), String> {
    let e: Vec<&str> = expected.collect();
    let a: Vec<&str> = actual.collect();
    let result = Comparison::new(&e, &a).compare().unwrap();
    if !result.is_empty() {
        Err(result
            .display(DisplayOptions { offset: 0, msg_fmt })
            .to_string())
    } else {
        Ok(())
    }
}

#[doc(hidden)]
pub fn inner_assert_diff(expected: Lines, actual: Lines, msg_fmt: &str) {
    if let Err(e) = inner_try_diff(expected, actual, msg_fmt) {
        panic!("{}", e)
    }
}

#[doc(hidden)]
pub fn inner_try_diff_file(
    expected: impl AsRef<Path>,
    actual: impl AsRef<Path>,
    msg_fmt: &str,
) -> Result<(), String> {
    let expected = expected.as_ref();
    let actual = actual.as_ref();
    let expected_contents = std::fs::read_to_string(expected)
        .unwrap_or_else(|e| panic!("Couldn't read expected file {}: {}", expected.display(), e));
    let actual_contents = std::fs::read_to_string(actual)
        .unwrap_or_else(|e| panic!("Couldn't read actual file {}: {}", actual.display(), e));

    inner_try_diff(expected_contents.lines(), actual_contents.lines(), msg_fmt)
}

#[doc(hidden)]
pub fn inner_assert_diff_file(expected: impl AsRef<Path>, actual: impl AsRef<Path>, msg_fmt: &str) {
    if let Err(e) = inner_try_diff_file(expected, actual, msg_fmt) {
        panic!("{}", e)
    }
}

#[cfg(feature = "dir_assert")]
mod dir_assert {
    use crate::inner_try_diff_file;
    use std::path::Path;
    use walkdir::WalkDir;

    /// Asserts equality between two directories, recursively.
    /// Two directories are considered equal iff they have exactly the same files and directories
    /// recursively and all corresponding files have exactly the same contents.
    /// Internally it uses [`try_diff_dir!`](macro.try_diff_dir.html) and then panics if directories are not equal.
    /// This macro requires that arguments implement trait `AsRef<Path>`
    ///
    /// *Note*: requires `dir_assert` feature enabled, e.g.
    ///
    /// // Cargo.toml
    /// [dependencies]
    /// diff_assert = { version = "*", features = ["dir_assert"]
    ///
    /// # Input
    /// `$expected` - path to the directory with expected content,
    /// `$actual` - path to the directory with actual content,
    /// `$message_args` - optional message when assertion fails.
    ///
    /// # Panics
    /// If expected directory content != actual directory content
    /// (either one directory have a file or sub-directory that is not present in the second directory
    /// or corresponding files have different content)
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # #[macro_use] extern crate diff_assert;
    /// # fn main() {
    /// let expected = "tests/data/diff_dir/different_file/a";
    /// let actual = "tests/data/diff_dir/different_file/b";
    ///
    /// assert_diff!(expected, actual, "Here is an optional message what has changed");
    /// # }
    /// ```
    #[macro_export]
    macro_rules! assert_diff_dir {
        ($expected: expr, $actual: expr) => { {
            let expected: &::std::path::Path = $expected.as_ref();
            let actual: &::std::path::Path = $actual.as_ref();
            $crate::assert_diff_dir!(expected, actual, "Found differences between {} and {}", expected.display(), actual.display())
        } };
        ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
            $crate::inner_assert_diff_dir($expected, $actual, &format!($message, $($message_args),*))
        };
    }

    /// Checks equality between two directories, recursively. Returns Err(String) if it fails.
    /// Two directories are considered equal iff they have exactly the same files and directories
    /// recursively and all corresponding files have exactly the same contents.
    /// This macro requires that arguments implement trait `AsRef<Path>`
    ///
    /// *Note*: requires `dir_assert` feature enabled, e.g.
    ///
    /// // Cargo.toml
    /// [dependencies]
    /// diff_assert = { version = "*", features = ["dir_assert"]
    ///
    /// # Input
    /// `$expected` - path to the directory with expected content,
    /// `$actual` - path to the directory with actual content,
    /// `$message_args` - optional message when assertion fails.
    ///
    /// # Panics
    /// If expected directory content != actual directory content
    /// (either one directory have a file or sub-directory that is not present in the second directory
    /// or corresponding files have different content)
    ///
    /// # Examples
    ///
    /// ```rust,should_panic
    /// # #[macro_use] extern crate diff_assert;
    /// # fn main() {
    /// let expected = "tests/data/diff_dir/different_file/a";
    /// let actual = "tests/data/diff_dir/different_file/b";
    ///
    /// let err = try_diff_dir!(expected, actual, "Here is an optional message what has changed").unwrap_err();
    ///
    /// assert_eq!("Here is an optional message what has changed", err);
    /// # }
    /// ```
    #[macro_export]
    macro_rules! try_diff_dir {
        ($expected: expr, $actual: expr) => { {
            let expected: &::std::path::Path = $expected.as_ref();
            let actual: &::std::path::Path = $actual.as_ref();
            $crate::try_diff_dir!(expected, actual, "Found differences between {} and {}", expected.display(), actual.display())
        } };
        ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
            $crate::inner_try_diff_dir($expected, $actual, &format!($message, $($message_args),*))
        };
    }

    #[doc(hidden)]
    pub fn inner_assert_diff_dir(
        expected: impl AsRef<Path>,
        actual: impl AsRef<Path>,
        msg_fmt: &str,
    ) {
        if let Err(e) = inner_try_diff_dir(expected, actual, msg_fmt) {
            panic!("{}", e)
        }
    }

    #[doc(hidden)]
    pub fn inner_try_diff_dir(
        expected_root: impl AsRef<Path>,
        actual_root: impl AsRef<Path>,
        msg_fmt: &str,
    ) -> Result<(), String> {
        let expected_root = expected_root.as_ref();
        let actual_root = actual_root.as_ref();

        let expected_walker = WalkDir::new(expected_root)
            .follow_links(true)
            .sort_by_file_name();
        let actual_walker = WalkDir::new(actual_root)
            .follow_links(true)
            .sort_by_file_name();

        for (expected, actual) in expected_walker.into_iter().zip(actual_walker.into_iter()) {
            let expected =
                expected.map_err(|e| format!("Couldn't read expected file or directory: {e}"))?;
            let actual =
                actual.map_err(|e| format!("Couldn't read actual file or directory {e}"))?;
            let relative_expected_path = expected
                .path()
                .strip_prefix(expected_root)
                .map_err(|e| format!("Couldn't find relative expected path: {e}"))?;
            let relative_actual_path = actual
                .path()
                .strip_prefix(actual_root)
                .map_err(|e| format!("Couldn't find relative actual path: {e}"))?;

            if relative_expected_path != relative_actual_path {
                return Err(format!(
                    "Inconsistent file and directory structure: {} vs {}",
                    expected.path().display(),
                    actual.path().display()
                ));
            }
            if expected.file_type() != actual.file_type() {
                return Err(format!(
                    "Inconsistent entry type. Expected {:?} got {:?} for {}",
                    expected.file_type(),
                    actual.file_type(),
                    relative_expected_path.display()
                ));
            }

            if expected.file_type().is_file() && actual.file_type().is_file() {
                inner_try_diff_file(expected.path(), actual.path(), msg_fmt)?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "dir_assert")]
pub use dir_assert::*;

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

    #[test]
    fn try_diff_file_same() {
        assert_diff_file!(
            "tests/data/diff_file/same/a.txt",
            "tests/data/diff_file/same/b.txt"
        )
    }

    #[test]
    fn try_diff_file_different() {
        let err = try_diff_file!(
            "tests/data/diff_file/different/a.txt",
            "tests/data/diff_file/different/b.txt"
        )
        .unwrap_err();

        assert!(
            err.trim().starts_with("Found differences"),
            "ERROR: {}",
            err
        );
    }

    #[cfg(feature = "dir_assert")]
    mod diff_dir {
        use super::*;

        #[test]
        fn try_diff_dir_same() {
            assert_diff_dir!("tests/data/diff_dir/same/a", "tests/data/diff_dir/same/b")
        }

        #[test]
        fn try_diff_dir_different_file() {
            let err = try_diff_dir!(
                "tests/data/diff_dir/different_file/a",
                "tests/data/diff_dir/different_file/b"
            )
            .unwrap_err();

            assert!(err.trim().starts_with("Found differences"));
        }

        #[test]
        fn try_diff_dir_different_structure() {
            let err = try_diff_dir!(
                "tests/data/diff_dir/different_structure/a",
                "tests/data/diff_dir/different_structure/b"
            )
            .unwrap_err();

            assert!(err
                .trim()
                .starts_with("Inconsistent file and directory structure"))
        }
    }
}
