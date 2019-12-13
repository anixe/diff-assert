mod diff;
pub use self::diff::*;
use itertools::Itertools;
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
macro_rules! assert_diff {
    ($expected: expr, $actual: expr) => {
        $crate::assert_diff!($expected, $actual, "Found differences")
    };
    ($expected: expr, $actual: expr, $message: literal $(,$message_args: expr)*) => {
        $crate::inner_assert_diff($expected.lines(), $actual.lines(), format!($message, $($message_args),*));
    };
}

#[doc(hidden)]
pub fn inner_assert_diff(expected: Lines, actual: Lines, msg_fmt: String) {
    let e: Vec<String> = expected.map(String::from).collect();
    let a: Vec<String> = actual.map(String::from).collect();
    let result = crate::diff_hunks(&e, &a, 3).unwrap();
    if !result.is_empty() {
        let mut msg = String::from("\n");
        msg += &msg_fmt;
        msg += "\n\n";

        msg += &result.into_iter().map(|s| s.to_string()).join("\n");

        panic!("{}", msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use test_case::test_case;

    #[test_case("A B C\nD E F", "A B D\nE F G")]
    #[test_case("A B C\nD E F", "A B D\nE F G\n1 2 3")]
    #[test_case(r#"|-------|-------------|-----------------------------------------|-----|---------------------------------|-----------|
| 24638 | Twin Room   | P1:3 A1:3 C0:2 FC0:9[0:1] MCA0:3[13:13] | DZ  | child_ages:["2:3","4:6","7:12"] |           |"#,
   r#"|-------|-------------|-----------------------------------------|-----|---------------------------|-----------|
| 24638 | Twin Room   | P1:3 A1:3 C0:2 FC0:9[0:3] MCA0:3[13:13] | DZ  | child_ages:["4:6","7:12"] |           |"#
    )]
    #[should_panic]
    fn test_diff_hunks(left: &str, right: &str) {
        let left: Vec<String> = left.lines().map(String::from).collect();
        let right: Vec<String> = right.lines().map(String::from).collect();
        let hunks = diff_hunks(&left, &right, 3).expect("hunks");

        if !hunks.is_empty() {
            let hunks = hunks.into_iter().map(|s| format!("{}\n", s)).join("\n");
            panic!("\n{}", hunks);
        }
    }

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
    #[should_panic]
    fn dbg_test() {
        let expected = ("Foo", "Bar");
        let actual = ("Foo", "foo");
        assert_dbg!(expected, actual);
    }
}
