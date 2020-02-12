[![Build Status](https://travis-ci.org/luke-biel/dir-assert.svg?branch=master)](https://travis-ci.org/luke-biel/dir-assert)

# dir-assert

Compare contents of two directories.

This crate provides macro and function for asserting whether two directories or files are equal.

Example usage:
```rust
#[test]
fn test_macro() {
    assert_paths!("actual", "expected");
}

#[test]
fn test_fn() {
    assert_paths("actual", "expected").unwrap();
}
```

These functions will panic yielding detailed information about which paths didn't match during comparison.
Moreover, line at which each file differs, when other with same name is found also will be reported.

To do before 1.0:
* [ ] assert should `println!` or `debug!` all paths it went through
* [x] we need to follow symlinks
* [ ] we need to add optional configuration to:
    * [ ] know if follow symlinks or just compare their names
    * [ ] compare `actual` with stringified dir (no `expected` folder, only &str as input)
    * [ ] provide file comparison function (we may use hashing comparers or just check file metadata)
* [ ] unit test error `new_*` functions
* [ ] change acceptance tests to compare `Debug` instead of `Display`
* [ ] hide `Debug` and `Clone` implementations behind feature

License: MIT
