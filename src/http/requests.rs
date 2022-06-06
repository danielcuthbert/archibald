/*
* Archibald: a loyal web server
* Main requests module
* Author: @danielcuthbert
*
*/

//use crate::http::methods::{Allowedmethods, MethodError};
use std::convert::TryFrom;
use crate::http::requests;
use crate::http::errors::ParseError;
use std::str;
use super::methods::{Allowedmethods, MethodError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Request {
    // We need to store the request body
    method: Allowedmethods,
    query: Option<String>, // This is a string that can be None
    path: String,
    body: String,
    statuscode: u16,
    statusmessage: String,
}

// We need to send back a response to the client 
// This will be a byte array to a string
// https://doc.rust-lang.org/std/convert/trait.From.html
// Note: This trait must not fail. If the conversion can fail, use TryFrom.

// example request
// GET /name?first=Daniel&last=Cuthbert HTTP/1.1 
// In order to get all of the request, we have to parse it word by word somehow 

// Convert a byte slice to a string

// TryFrom returns a result and this might fail so we can use this to handle errors
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // this is the actual function that does stuff. copied from the docs. 
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // We need to match on the results, so use match

let request = str::from_utf8(buf)?;
// variable shadowing is where you re-use the same variable name (request) but overwrite it with a different value
let (method, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
let (path, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
let (protocol, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;

// We only support HTTP/1.1 right now, so return an error if it's not that
if protocol != "HTTP/1.1" {
    return Err(ParseError::InvalidProtocol);
}

let method = method.parse()?; // convets from string to enum type


// basically accepts the request, which is a string slice 
// so get 'method', 'path', 'query', 'body' etc 
// we need a loop here to get all of the request
// I don't think this is the best way to do this at all and probably breaks shit


// We dont check for carriage returns or newlines here because we're not doing anything with the request body but we should do
fn parse_request(request: &str) -> Option<(&str, &str)>  {
//     let mut iterate = request.chars();
//     loop {
//         let mut current = iterate.next();
//         if current == None {
//             break;
//         }
//         let mut next = iterate.next();
//         if next == None {
//             break;
//         }
//         if current == Some(' ') && next == Some(' ') {
//             break;
//         }

//         for d in request.chars() {
//             if d == ' ' {
//                 break;
//             }
//         }
//     }
// }

pub emum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidVersion,
    InvalidHeader,
    InvalidBody,
    InvalidProtocol,
    InvalidEncoding
}

impl ParseError{

    fn description(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid Request",
            ParseError::InvalidMethod => "Invalid Method",
            ParseError::InvalidVersion => "Invalid Version",
            ParseError::InvalidHeader => "Invalid Header",
            ParseError::InvalidBody => "Invalid Body",
            ParseError::InvalidProtocol => "Invalid Protocol",
            ParseError::InvalidEncoding => "Invalid encoding"
        }
    }

}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> ParseError {
        ParseError::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> ParseError {
        ParseError::InvalidMethod
    }
}

}