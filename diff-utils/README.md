# diff_utils
[![Crates.io](https://img.shields.io/crates/v/diff-utils.svg)](https://crates.io/crates/diff-utils)
[![Docs.rs](https://docs.rs/diff-utils/badge.svg)](https://docs.rs/diff-utils)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rust-lang/docs.rs/master/LICENSE)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

## About this crate
It is a hearth of our [`diff-assert`](../diff-assert/index.html) crate. It contains all logic
necessary to compare two text files and to produce diff or patch file.
It uses `diffs` library under the hood to perform patience algorithm.

## Usage

The most important structure is [`Comparison`](struct.Comparison.html). It gives you nice
interface for comparing string slices:

```rust
use diff_utils::Comparison;
let result = Comparison::new(&["foo", "bar"], &["foo", "foo"]).compare().expect("Comparison failed");
```

The result can be either printed (with `display` feature - see [`display`](struct.CompareResult#method.display) method),
used to generate patch (with `patch` feature - see [`patch`](struct.CompareResult#method.patch) method), or to simply
check if two files were equal:

```rust
if result.is_empty() {
    println!("Files have identical content");
}
```

## Features:
* `display` - to pretty print hunks in the console,
* `patch` to generate patch files

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
