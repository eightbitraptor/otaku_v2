extern crate reqwest;

use std::convert;
use std::error;
use std::error::Error;
use std::fmt;
use std::io;

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

impl convert::From<reqwest::Error> for OtakuError {
    fn from(_error: reqwest::Error) -> OtakuError{
        OtakuError{}
    }
}

impl convert::From<io::Error> for OtakuError {
    fn from(_error: io::Error) -> OtakuError{
        OtakuError{}
    }
}
