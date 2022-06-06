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

use::std::error::Error;
use std::fmt;
use::std::fmt::Display;
use::std::fmt::Result;
use::std::str::Utf8Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError {
NotFound,
InvalidRequest,
InvalidMethod,
InvalidVersion,
InvalidHeader,
InvalidBody,
InvalidProtocol,
InvalidEncoding
}

// We can use the std crate for errors 

impl fmt::Display for ParseError {
    fn wah(&self) -> &str {
        match self {
            ParseError::NotFound => "Not Found",
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidMethod => "Invalid Method",
            ParseError::InvalidVersion => "Invalid Version",
            ParseError::InvalidHeader => "Invalid Header",
            ParseError::InvalidBody => "Invalid Body",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidEncoding => "Invalid encoding"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ParseError::NotFound => None,
            ParseError::InvalidRequest => None,
            ParseError::InvalidMethod => None,
            ParseError::InvalidVersion => None,
            ParseError::InvalidHeader => None,
            ParseError::InvalidBody => None,
            ParseError::InvalidProtocol => None,
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        todo!()
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
        write!(f, "{}", self.description())
    }
}
    
