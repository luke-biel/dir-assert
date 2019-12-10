#![feature(try_trait)]
//! Compare contents of two directories.
//!
//!

mod assert_dirs;
mod error;
mod test_result;

pub use crate::assert_dirs::assert_dirs;
pub use crate::error::Error;
pub use crate::test_result::TestResult;

#[macro_export]
macro_rules! assert_dirs {
    (actual: expr, expected: expr) => {
        let test_result = dir_assert::assert_dirs(actual, expected);

        assert!(
            !test_result.result,
            "\nfile mismatch:\n{:#?}\n",
            test_result.errors
        );
    };
}
