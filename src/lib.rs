//! Compare contents of two directories.

mod assert_paths;
mod error;

pub use crate::assert_paths::assert_paths;
pub use crate::error::Error;

#[macro_export]
macro_rules! assert_paths {
    ($actual: expr, $expected: expr) => {{
        match $crate::assert_paths($actual, $expected) {
            Ok(_) => { /* do nothing */ }
            Err(test_result) => panic!("file mismatch\n{:#?}", test_result),
        }
    }};
}
