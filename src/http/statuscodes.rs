/// This is where we store all the codes we want to send
/// HTTP status codes are cool https://www.restapitutorial.com/httpstatuscodes.html
use std::fmt::{Display, Formatter, Result as FmtResult}; //renaming result as i mess this up a lot so this makes it easier to find it

// This is where we store all the codes we want to send
// We have a friend name and then the integer that we can cast using StatusCode as a u16
// It defines an enum StatusCode that represents HTTP status codes and provides implementations for the Into, Display, and Debug traits. The http_status_reason_phrase method returns the HTTP status reason phrase for a given status code. The Display trait implementation allows the StatusCode enum to be displayed as a string. 
// The Debug trait implementation allows the StatusCode enum to be printed in a debug format.




#[derive(Debug, Clone, Copy)] // we need this to copy our string and clone it
#[repr(u16)] //treat all of these as u16s
pub enum StatusCode {
    /// This is where we store all the codes we want to send
    JollyGood = 200,
    BadRequest = 400,
    FORBIDDEN = 403,
    NotFound = 404,
    IAmATeapot = 418,
    InternalServerError = 500,
}

impl Into<u16> for StatusCode {
    /// An implementation of the Into trait for the StatusCode enum
    fn into(self) -> u16 {
        self as u16
    }
}

// I guess we need some method to map these to reasons why the errors are being sent

impl StatusCode {
    /// The implementation of the StatusCode enum
    pub fn http_status_reason_phrase(&self) -> &str {
        match self {
            // self is a receiver of a method so we can just match on it
            Self::JollyGood => "Jolly Good",
            Self::BadRequest => "Bad Request",
            Self::FORBIDDEN => "Forbidden",
            Self::NotFound => "Not Found",
            Self::IAmATeapot => "I am a teapot",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}

// How do you display the actual status code itself? We need to implement the Display trait
// impl is used to implement some kind of functionality for a type.
// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html
// the integer needs to be unsigned (cannot be a negative number) so u16

// types that live on the stack can be copied and moved around freely (for example, integers)
// types that live on the heap cannot be copied or moved around freely (for example, strings)
// https://users.rust-lang.org/t/cant-derive-copy-because-of-string/18665
// For this, we have to use the clone function

impl Display for StatusCode {
    /// Here is the implementation of the Display trait for the StatusCode enum
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // use self, f to get the type of the struct and then
        write!(f, "{}", *self as u16) // we use write to write to the formatter. This is a format string
    }
}
