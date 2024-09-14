use super::methods::MethodError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io;
use std::str::Utf8Error;

#[non_exhaustive]
pub enum ParseError {
    NotFound(u16),
    InvalidRequest,
    InvalidMethod,
    InvalidHeader,
    InvalidBody,
    InvalidEncoding,
    InvalidProtocol,
    InvalidPath,
    InternalServerError,
    MethodNotAllowed,
    IOError(String),
    // Add other variants as needed
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use ParseError::*;
        match self {
            InvalidRequest => write!(f, "Invalid Request"),
            InvalidMethod => write!(f, "Invalid Method"),
            InvalidHeader => write!(f, "Invalid Header"),
            InvalidBody => write!(f, "Invalid Body"),
            InvalidEncoding => write!(f, "Invalid Encoding"),
            InvalidProtocol => write!(f, "Invalid Protocol"),
            InvalidPath => write!(f, "Invalid Path"),
            InternalServerError => write!(f, "Internal Server Error"),
            MethodNotAllowed => write!(f, "Method Not Allowed"),
            NotFound(code) => write!(f, "Not Found ({})", code),
            IOError(message) => write!(f, "IO Error: {}", message),
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
