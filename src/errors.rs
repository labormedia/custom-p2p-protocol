pub use std::error::Error;
use core::fmt;

#[derive(Debug)]
pub enum ErrorSide {
    PayloadSizeMismatch(usize),
    Unreachable,
    InvalidIPv6Segments,
    StdError(Box<dyn Error>)
}

impl fmt::Display for ErrorSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSide::PayloadSizeMismatch(size) => write!(f, "Payload Size Mismatch : {:?}.", size),
            ErrorSide::Unreachable => write!(f, "Unreachable code."),
            ErrorSide::InvalidIPv6Segments => write!(f, "Invalid IPv6 segments."),
            ErrorSide::StdError(error) => write!(f, "Std Error : {}", error),
        }
        
    }
}

impl Error for ErrorSide {}

impl From<Box<dyn Error>> for ErrorSide {
    fn from(boxed_error: Box<dyn Error>) -> Self {
        ErrorSide::StdError(boxed_error)
    }
}

impl From<std::io::Error> for ErrorSide {
    fn from(error: std::io::Error) -> Self {
        ErrorSide::StdError(Box::new(error))
    }
}