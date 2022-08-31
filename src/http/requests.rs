/*
* Archibald: a loyal web server
* Main requests module
* Author: @danielcuthbert
*
* This code serves as the request function.
*/

use super::methods::{Allowedmethods, MethodError};
use crate::http::errors::ParseError;
//use crate::http::requests;
use super::{QueryString, ValueofQueryString};
use log::{debug, error, info, trace, warn};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use std::str;
// This function stores the request body we will use

#[derive(Debug)]
pub struct Request {
    query: Option<QueryString>, // This is a string that can be None
    path: String,
    body: String,
    statuscode: u16,
    statusmessage: String,
    method: Allowedmethods,
}

// We need to send back a response to the client
// This will be a byte array to a string
// https://doc.rust-lang.org/std/convert/trait.From.html
// Note: This trait must not fail. If the conversion can fail, use TryFrom.

// example request
// GET /name?first=Daniel&last=Cuthbert HTTP/1.1
// In order to get all of the request, we have to parse it word by word somehow

impl Request {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Allowedmethods {
        &self.method
    }

    pub fn query(&self) -> &Option<QueryString> {
        &self.query
    }
}

// TryFrom returns a result and this might fail so we can use this to handle errors
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // this is the actual function that does stuff. copied from the docs.
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // We need to match on the results, so use match

        let request: &str = str::from_utf8(buf).map_err(|_| ParseError::InvalidEncoding)?;
        // variable shadowing is where you re-use the same variable name (request) but overwrite it with a different value
        let (method, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;

        // We only support HTTP/1.1 right now, so return an error if it's not that
        if protocol != "HTTP/1.1" {
            //return Err(ParseError::InvalidProtocol);
            error!("[*] Archibald: Invalid protocol: {}", protocol);
        }

        let method: Allowedmethods = method.parse().map_err(|_| ParseError::InvalidMethod)?;
        // convets from string to enum type
        //todo!("Please fix this :)");

        let query_str: Option<&str> = None;
        let mut query = None;
        // we want to match on something but not the other variants
        match path.find('?') {
            Some(index) => {
                query = Some(QueryString::from(&path[index + 1..])); // representing 1 byte after the '?'
                path = &path[..index]; // representing the path up to the '?'
            }
            None => {}
        }

        // Use fmt::display for parse errors
        // https://doc.rust-lang.org/std/fmt/trait.Display.html
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
        pub enum ParseError {
            InvalidRequest,  // This is the error that is returned if the request is invalid
            InvalidEncoding, // This is the error that is returned if the encoding is invalid
            InvalidProtocol, // This is the error that is returned if the protocol is invalid
            InvalidMethod,   // This is the error that is returned if the method is invalid
        }
        // Now we need to implement this
        impl ParseError {
            fn description(&self) -> &str {
                match self {
                    Self::InvalidRequest => "Invalid Request",
                    Self::InvalidEncoding => "Invalid Encoding",
                    Self::InvalidProtocol => "Invalid Protocol",
                    Self::InvalidMethod => "Invalid Method",
                }
            }
        }
    }
}
impl Error for ParseError {}

// basically accepts the request, which is a string slice
// so get 'method', 'path', 'query', 'body' etc
// we need a loop here to get all of the request
// I don't think this is the best way to do this at all and probably breaks shit

// We dont check for carriage returns or newlines here because we're not doing anything with the request body but we should do
fn parse_request(request: &str) -> Option<(&str, &str)> {
    // todo!("I really need to do something here")
    // Take the request and string and then add 1 to the index to get the next character
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}
