use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Error {
    Critical(String),
    ExtraExpected(PathBuf),
    ExtraActual(PathBuf),
    InvalidComparison { expected: PathBuf, actual: PathBuf },
    FileContentsMismatch { expected: PathBuf, actual: PathBuf },
    MissingPath(PathBuf),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Critical(msg) => write!(f, "critical error has occurred, {}", msg),
            Error::ExtraExpected(expected) => write!(
                f,
                "found expected file {:?} with no counterpart in actual",
                expected
            ),
            Error::ExtraActual(actual) => write!(
                f,
                "found actual file {:?} with no counterpart in expected",
                actual
            ),
            Error::InvalidComparison { expected, actual } => write!(
                f,
                "comparing directories and files will not work with {:?} and {:?}",
                actual, expected
            ),
            Error::FileContentsMismatch { actual, expected } => write!(
                f,
                "files {:?} and {:?} have different contents",
                actual, expected
            ),
            Error::MissingPath(path) => write!(f, "path {:?} not found", path),
        }
    }
}

impl Error {
    pub fn new_critical<S: Into<String>>(message: S) -> Self {
        Error::Critical(message.into())
    }

    pub fn new_extra_expected<P: Into<PathBuf>>(path: P) -> Self {
        Error::ExtraExpected(path.into())
    }

    pub fn new_extra_actual<P: Into<PathBuf>>(path: P) -> Self {
        Error::ExtraActual(path.into())
    }

    pub fn new_invalid_comparison<PE: Into<PathBuf>, PA: Into<PathBuf>>(
        expected: PE,
        actual: PA,
    ) -> Self {
        Error::InvalidComparison {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn new_file_contents_mismatch<PE: Into<PathBuf>, PA: Into<PathBuf>>(
        expected: PE,
        actual: PA,
    ) -> Self {
        Error::FileContentsMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn new_missing_path<P: Into<PathBuf>>(path: P) -> Self {
        Error::MissingPath(path.into())
    }
}