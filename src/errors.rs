pub use std::error::Error;
use core::fmt;

#[derive(Debug)]
pub enum ErrorSide {
    PayloadSizeMismatch
}

impl fmt::Display for ErrorSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorSide.")
    }
}

impl Error for ErrorSide {}