pub use diff_utils::*;
use std::str::Lines;

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
            $message, $($message_args),*)
    }
}

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
            $message, $($message_args),*)
    }
}

#[macro_export]
macro_rules! try_diff {
    ($expected: expr, $actual: expr) => {
        $crate::try_diff!($expected, $actual, "Found differences")
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::inner_try_diff($expected.lines(), $actual.lines(), format!($message, $($message_args),*))
    };
}

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
