use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;
use std::io;
use super::methods::MethodError;

#[non_exhaustive]
pub enum ParseError {
    NotFound(u16),
    InvalidRequest,
    InvalidMethod,
    InvalidHeader,
    InvalidBody,
    InvalidEncoding,
    InvalidProtocol,
    IOError(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use ParseError::*;
        match self {
            InvalidRequest => write!(f, "Invalid Request"),
            InvalidMethod => write!(f, "Invalid Method"),
            // ... other cases ...
            IOError(message) => write!(f, "IO Error: {}", message),
            NotFound(_) => todo!(),
            InvalidHeader => todo!(),
            InvalidBody => todo!(),
            InvalidEncoding => todo!(),
            InvalidProtocol => todo!(),
            // ... other cases ...
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ParseError: {}", self)
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IOError(err.to_string())
    }
}

impl Error for ParseError {}
