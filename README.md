# diff_assert
[![Crates.io](https://img.shields.io/crates/v/diff-assert.svg)](https://crates.io/crates/diff-assert)
[![Docs.rs](https://docs.rs/diff-assert/badge.svg)](https://docs.rs/diff-assert)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rust-lang/docs.rs/master/LICENSE)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

## About this crate
We designed this crate to replace `assert_eq!` with more powerful `assert_diff!` which can show
exactly what part of actual data differs from expected behaviour.

You may wonder why another crate like this. We noticed that other crates have significant issues
with for example whitespaces and newlines, which brings us to this solution. It's well tested,
used in dozens of thousands cases in our ecosystem, and we believe it is simply more correct.

## How to use it?

The simplest example is to use [`assert_diff!`](macro.assert_diff.html) macro:
```rust
let expected = r#"foo
bar"#;

let actual = r#"foo
foo"#;

assert_diff!(expected, actual, "Here is an optional message what has changed");
```

Another possibility is to use [`try_diff!`](macro.try_diff.html) macro if you don't want to panic.
It returns nice `Result<(), String>` instead.
```rust
let expected = r#"foo
bar"#;

let actual = r#"foo
foo"#;

if let Err(e) = try_diff!(expected, actual, "Here is an optional message what has changed") {
    eprintln!("Oh nay, we got a difference!");
    eprintln!("{}", e);
}
```

Besides that, crate also contains similar macros for comparing not string slices but
[`Debug`](std::fmt::Debug) format outputs. It is quite handy for testing intermediate outputs.
* [`assert_dbg!`](macro.assert_dbg.html)
* [`try_dbg!`](macro.try_dbg.html)

## Contribution
Please if possible use `.hooks/`:
```bash
cp .hooks/* .git/hooks/
```

All issues and pull requests are welcomed :)

Recommended handy tools:
* `cargo readme` - to generate README.md,
* `cargo fmt` - to reformat workspace,
* `cargo edit` - to add/remove dependencies,
* `cargo clippy` - to maintain code quality,
* `cargo fix` - to fix errors and warnings.
