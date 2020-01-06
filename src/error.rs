use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Error {
    /// Error with system IO, unrecoverable, ceasing traverse of current directory
    Critical(String),
    /// Extra expected item exists
    ExtraExpected(PathBuf),
    /// Extra actual item exists
    ExtraActual(PathBuf),
    /// Found filename and directory sharing same name and path
    InvalidComparison { expected: PathBuf, actual: PathBuf },
    /// Two files with same path have different contents
    FileContentsMismatch {
        expected: PathBuf,
        actual: PathBuf,
        line: usize,
    },
    /// Top level directories are missing (eg. actual folder wasn't actually created)
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
            Error::FileContentsMismatch {
                actual,
                expected,
                line,
            } => write!(
                f,
                "files {:?} and {:?} differ on line {}",
                actual, expected, line
            ),
            Error::MissingPath(path) => write!(f, "path {:?} not found", path),
        }
    }
}

impl Error {
    pub(crate) fn new_critical<S: Into<String>>(message: S) -> Self {
        Error::Critical(message.into())
    }

    pub(crate) fn new_extra_expected<P: Into<PathBuf>>(path: P) -> Self {
        Error::ExtraExpected(path.into())
    }

    pub(crate) fn new_extra_actual<P: Into<PathBuf>>(path: P) -> Self {
        Error::ExtraActual(path.into())
    }

    pub(crate) fn new_invalid_comparison<PE: Into<PathBuf>, PA: Into<PathBuf>>(
        expected: PE,
        actual: PA,
    ) -> Self {
        Error::InvalidComparison {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub(crate) fn new_file_contents_mismatch<PE: Into<PathBuf>, PA: Into<PathBuf>>(
        expected: PE,
        actual: PA,
        line: usize,
    ) -> Self {
        Error::FileContentsMismatch {
            expected: expected.into(),
            actual: actual.into(),
            line,
        }
    }

    pub(crate) fn new_missing_path<P: Into<PathBuf>>(path: P) -> Self {
        Error::MissingPath(path.into())
    }
}

// TODO: Test if errors created are correct, not if they **Display** correct thing
// TODO: Test if errors are Stringified correctly
