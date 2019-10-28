mod diff;
pub use self::diff::*;

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
macro_rules! assert_diff {
    ($expected: expr, $actual: expr) => {
        $crate::assert_diff!($expected, $actual, "Found differences")
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        let e: Vec<String> = $expected.lines().map(String::from).collect();
        let a: Vec<String> = $actual.lines().map(String::from).collect();
        let result = $crate::diff(&e, &a, 3).unwrap();
        if !result.is_empty() {
            let mut msg = String::from("\n");
            msg += &format!($message, $($message_args),*);
            msg += "\n\n";
            for s in result {
                msg += &format!("{}\n", s);
            }

            panic!("{}", msg)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test() {
        let expected = "foo
        bar".to_string();

        let actual = "foo
        foo".to_string();

        assert_diff!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn dbg_test() {
        let expected = ("Foo", "Bar");
        let actual = ("Foo", "foo");
        assert_dbg!(expected, actual);
    }
}