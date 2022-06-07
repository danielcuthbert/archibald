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
use ::std::fmt::Display;
use ::std::fmt::Result;
use ::std::str::Utf8Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
    NotFound,
    InvalidRequest,
    InvalidMethod,
    InvalidVersion,
    InvalidHeader,
    InvalidBody,
    InvalidProtocol,
    InvalidEncoding,
}

// Using UTF8 for the errors, we need to wrangle that into our ParseError somehow
// this function will receive the error as a utf8 as a parameter and then push it into the ParseError enum

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> ParseError {
        ParseError::InvalidEncoding
    }
}

// We need to display the errors. The std crate has a Display trait that we can use to display the errors well

impl Display for ParseError {
    // This is the function that will be called when we print the error
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NotFound => write!(f, "Not Found"),
            ParseError::InvalidRequest => write!(f, "Invalid Request"),
            ParseError::InvalidMethod => write!(f, "Invalid Method"),
            ParseError::InvalidVersion => write!(f, "Invalid Version"),
            ParseError::InvalidHeader => write!(f, "Invalid Header"),
            ParseError::InvalidBody => write!(f, "Invalid Body"),
            ParseError::InvalidProtocol => write!(f, "Invalid Protocol"),
            ParseError::InvalidEncoding => write!(f, "Invalid encoding"),
        }
    }
}
