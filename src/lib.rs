mod diff;
pub use self::diff::*;

#[macro_export]
macro_rules! assert_diff {
    ($expected: expr, $actual: expr) => {
        $crate::assert_diff!($expected, $actual, "Found differences")
    };
    ($expected: expr, $actual: expr, $($message: tt),*) => {
        let e: Vec<String> = $expected.lines().map(String::from).collect();
        let a: Vec<String> = $actual.lines().map(String::from).collect();
        let result = $crate::diff(&e, &a, 3).unwrap();
        if !result.is_empty() {
            let mut msg = String::from("\n");
            msg += &format!($($message),*);
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
}