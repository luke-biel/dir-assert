use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Error {
    Critical(String),
    ExtraExpected(PathBuf),
    ExtraActual(PathBuf),
    InvalidComparison { expected: PathBuf, actual: PathBuf },
    FileContentsMismatch { expected: PathBuf, actual: PathBuf },
}

impl From<String> for Error {
    fn from(string: String) -> Self {
        Error::Critical(string)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Critical(msg) => write!(f, "critical error has occurred, {}", msg),
            Error::ExtraExpected(expected) => write!(
                f,
                "found expected file with no counterpart in actual, {:?}",
                expected
            ),
            Error::ExtraActual(actual) => write!(
                f,
                "found actual file with no counterpart in expected, {:?}",
                actual
            ),
            Error::InvalidComparison { expected, actual } => write!(
                f,
                "comparing directories and files will not work, {:?} =/= {:?}",
                actual, expected
            ),
            Error::FileContentsMismatch { actual, expected } => write!(
                f,
                "files {:?} and {:?} have different contents",
                actual, expected
            ),
        }
    }
}
