use std::error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct OtakuError {}

impl error::Error for OtakuError {
    fn description(&self) -> &str {
        "Oops, something went wrong"
    }
}

impl fmt::Display for OtakuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Otaku Error: {}", self.description())
    }
}
