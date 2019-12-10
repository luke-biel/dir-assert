use crate::Error;
use std::ops::Try;

#[derive(Debug)]
pub struct TestResult {
    pub result: bool,
    pub errors: Vec<Error>,
}

impl From<String> for TestResult {
    fn from(critical_error: String) -> Self {
        Self {
            result: false,
            errors: vec![Error::Critical(critical_error)],
        }
    }
}

impl From<Error> for TestResult {
    fn from(error: Error) -> Self {
        Self {
            result: false,
            errors: vec![error],
        }
    }
}

impl Try for TestResult {
    type Ok = ();
    type Error = Error;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        if self.result {
            Ok(())
        } else {
            Err(self.errors.first().unwrap().clone()) // If result is false, this must contain at least one element
        }
    }

    fn from_error(error: Self::Error) -> Self {
        Self {
            result: false,
            errors: vec![error],
        }
    }

    fn from_ok(_: Self::Ok) -> Self {
        Self {
            result: true,
            errors: vec![],
        }
    }
}
