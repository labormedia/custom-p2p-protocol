pub use std::error::Error;
use core::fmt;

#[derive(Debug)]
pub enum ErrorSide {
    PayloadSizeMismatch(Box<[u8]>),
    Unreachable,
}

impl fmt::Display for ErrorSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSide::PayloadSizeMismatch(size) => write!(f, "Payload Size Mismatch : {:?}.", size),
            ErrorSide::Unreachable => write!(f, "Unreachable code."),
        }
        
    }
}

impl Error for ErrorSide {}