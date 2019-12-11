//! Compare contents of two directories.

mod assert_paths;
mod error;

pub use crate::assert_paths::assert_paths;
pub use crate::error::Error;

#[macro_export]
macro_rules! assert_paths {
    (actual: expr, expected: expr) => {
        let test_result = $crate::assert_paths(actual, expected);

        assert!(
            test_result.is_empty(),
            "\nfile mismatch:\n{:#?}\n",
            test_result
        );
    };
}
