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
use::std::fmt::Display;
use::std::fmt::Result;


pub enum ParseError {
NotFound,
InvalidRequest,
InvalidMethod,
InvalidVersion,
InvalidHeader,
InvalidBody,
InvalidProtocol,
}

// We can use the std crate for errors 

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::NotFound => "Not Found",
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidMethod => "Invalid Method",
            ParseError::InvalidVersion => "Invalid Version",
            ParseError::InvalidHeader => "Invalid Header",
            ParseError::InvalidBody => "Invalid Body",
            ParseError::InvalidProtocol => "Invalid Protocol",
        }
    }
}
    
// We need to display the errors. The std crate has a Display trait that we can use to display the errors well

impl Display for ParseError {
    // This is the function that will be called when we print the error
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
    
