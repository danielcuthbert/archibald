/*
* Archibald: a loyal web server
* Main error module
* Author: @danielcuthbert
* Rust requires you to expect errors and do something about them before they occur
* Rust has two types of errors: recoverable and unrecoverable
* Recoverable errors are errors that can be handled by the user
* Unrecoverable errors are errors that cannot be handled by the user
* Rust uses enum for recoverable errors
*/

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

use super::methods::MethodError;

#[non_exhaustive]
pub enum ParseError {
    /// This is the error that is returned when a number of things go wrong with the HTTP request
    NotFound(u16),
    InvalidRequest,
    InvalidMethod,
    InvalidHeader,
    InvalidBody,
    InvalidEncoding,
    InvalidProtocol,
}

// Using UTF8 for the errors, we need to wrangle that into our ParseError somehow
// this function will receive the error as a utf8 as a parameter and then push it into the ParseError enum

impl Display for ParseError {
    /// The implementation of the Display trait for the ParseError enum
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use ParseError::*;
        match self {
            InvalidRequest => write!(f, "Invalid Request"),
            InvalidMethod => write!(f, "Invalid Method"),
            InvalidHeader => write!(f, "Invalid Header"),
            InvalidBody => write!(f, "Invalid Body"),
            InvalidEncoding => write!(f, "Invalid Encoding"),
            NotFound(status) => write!(f, "Not Found: {}", status),
            InvalidProtocol => write!(f, "Invalid Protocol"),
        }
    }
    // write!(f, "ParseError: {}", self.description());
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "ParseError: {}", self.description())
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
// This represents how we handle different error messages

impl ParseError {
    fn description(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidBody => "Invalid Body",
            Self::InvalidHeader => "Invalid Header",
            Self::NotFound(_) => "Not Found",
            Self::InvalidProtocol => "Invalid Protocol",
            ParseError::NotFound(_) => todo!(),
            ParseError::InvalidRequest => todo!(),
            ParseError::InvalidMethod => todo!(),
            ParseError::InvalidHeader => todo!(),
            ParseError::InvalidBody => todo!(),
            ParseError::InvalidEncoding => todo!(),
            ParseError::InvalidProtocol => todo!(),
        }
    }
}

// this is to handle utf8 errors
// it accepts the utf8 error as a parameter and then pushes it into the ParseError enum

impl Error for ParseError {}
