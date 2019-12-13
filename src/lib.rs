//! Compare contents of two directories.
//!
//! This crate provides macro and function for asserting whether two directories or files are equal.
//!

mod assert_paths;
mod error;

pub use crate::assert_paths::assert_paths;
pub use crate::error::Error;

/// Recursively scan contents of two directories and find differences.
///
/// eg.:
/// ```rust,ignore
/// #[test]
/// fn should_directories_be_equal() {
///     assert_paths!("actual", "expected");
/// }
/// ```
///
/// This macro will panic if directories "actual" and "expected" differ at any depth.
///
/// It can accept both file names and directory names as arguments.
#[macro_export]
macro_rules! assert_paths {
    ($actual: expr, $expected: expr) => {{
        match $crate::assert_paths($actual, $expected) {
            Ok(_) => { /* do nothing */ }
            Err(test_result) => panic!("file mismatch\n{:#?}", test_result),
        }
    }};
}
