#[derive(Debug, Clone)]
pub enum Error {
    Critical(String),
    ExtraExpected(String),
    ExtraActual(String),
    ContentMismatch(String),
}

impl From<String> for Error {
    fn from(string: String) -> Self {
        Error::Critical(string)
    }
}
