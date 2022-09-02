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

use ::std::error::Error;
use std::fmt::{Debug, Display, Result as FmtResult};
use std::str::Utf8Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
    NotFound,        // This is the error that is returned when the request is not found
    InvalidRequest,  // This is the error that is returned when the request is invalid
    InvalidMethod,   // This is the error that is returned when the method is invalid
    InvalidVersion,  // This is the error that is returned when the version is invalid
    InvalidHeader,   // This is the error that is returned when the header is invalid
    InvalidBody,     // This is the error that is returned when the body is invalid
    InvalidProtocol, // This is the error that is returned when the protocol is invalid
    InvalidEncoding, // This is the error that is returned when the encoding is invalid
}

// Using UTF8 for the errors, we need to wrangle that into our ParseError somehow
// this function will receive the error as a utf8 as a parameter and then push it into the ParseError enum

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        write!(f, "ParseError: {}", self.description())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        write!(f, "ParseError: {}", self.description())
    }
}
// This represents how we handle different error messages

// Now we need to implement this
impl ParseError {
    fn description(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidBody => "Invalid Body",
            Self::InvalidHeader => "Invalid Header",
            Self::InvalidVersion => "Invalid Version",
            Self::NotFound => "Not Found",
        }
    }
}

// this is to handle utf8 errors
// it accepts the utf8 error as a parameter and then pushes it into the ParseError enum

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Error for ParseError {}
