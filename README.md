# diff-assert
## Basic usage:

```rust
use diff_assert::assert_diff;
let expected = r#"foo
bar"#.to_string();

let actual   = r#"foo
foo"#.to+string();

assert_diff!(expected, actual, "message");
```

If you do not want to panic:

```rust
use diff_assert::try_diff;
let expected = r#"foo
bar"#.to_string();

let actual   = r#"foo
foo"#.to+string();

if let Err(e) =  try_diff!(expected, actual, "message") {
    eprintln!("{}", e);
}
```
