//! Compare contents of two directories.
//!
//! This crate provides macro and function for asserting whether two directories or files are equal.
//!
//! Example usage:
//! ```rust,ignore
//! #[test]
//! fn test_macro() {
//!     assert_paths!("actual", "expected");
//! }
//!
//! #[test]
//! fn test_fn() {
//!     assert_paths("actual", "expected").unwrap();
//! }
//! ```
//!
//! These functions will panic yielding detailed information about which paths didn't match during comparison.
//! Moreover, line at which each file differs, when other with same name is found also will be reported.
//!
//! To do before 1.0:
//! * assert should `println!` or `debug!` all paths it went through
//! * we need to follow symlinks
//! * we need to add optional configuration to:
//!     * know if follow symlinks or just compare their names
//!     * compare `actual` with stringified dir (no `expected` folder, only &str as input)
//!     * provide file comparison function (we may use hashing comparers or just check file metadata or even omit some data like timestamps)
//! * unit test error `new_*` functions
//! * change acceptance tests to compare `Debug` instead of `Display`
//! * hide `Debug` and `Clone` implementations behind feature

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
