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


License: MIT
