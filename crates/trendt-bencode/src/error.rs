use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Unexpected end of input
    UnexpectedEof,
    /// Invalid character encountered
    InvalidCharacter(u8),
    /// Invalid integer format
    InvalidInteger,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEof => write!(f, "unexpected end of input"),
            Error::InvalidCharacter(c) => write!(f, "invalid character: {}", *c as char),
            Error::InvalidInteger => write!(f, "invalid integer format"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
