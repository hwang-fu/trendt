use std::fmt;

use serde::{de, ser};

#[derive(Debug)]
pub enum Error {
    /// Unexpected end of input
    UnexpectedEof,
    /// Invalid character encountered
    InvalidCharacter(u8),
    /// Invalid integer format
    InvalidInteger,
    /// Dictionary keys must be byte strings
    InvalidDictKey,
    /// Dictionary keys must be sorted
    UnsortedDictKeys,

    /// Custom message from serde
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEof => write!(f, "unexpected end of input"),
            Error::InvalidCharacter(c) => write!(f, "invalid character: {}", *c as char),
            Error::InvalidInteger => write!(f, "invalid integer format"),
            Error::InvalidDictKey => write!(f, "dictionary keys must be byte strings"),
            Error::UnsortedDictKeys => write!(f, "dictionary keys must be sorted"),

            Error::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
