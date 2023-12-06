use std::fmt;

#[derive(Debug)]
pub struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CustomError {}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError(err.to_string())
    }
}

impl CustomError {
    pub fn new(msg: &str) -> CustomError {
        CustomError(msg.into())
    }
}
