/*
* Archibald: a loyal web server
* Main requests module
* Author: @danielcuthbert
*
* This code serves as the request function.
*/

use super::methods::{Allowedmethods, MethodError};
use super::{QueryString, ValueofQueryString};
use crate::http::errors::ParseError;
use log::{debug, error, info, trace, warn};
use std::convert::TryFrom;
use std::intrinsics::const_eval_select;
use std::str;

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

// Handle utf-8 errors
// this bit is frustrating as hell and hurting me more than it should.
// I know I need to return something but I don't know what

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        let (method, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (statuscode, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (statusmessage, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (body, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
    }
}

// We dont check for carriage returns or newlines here because we're not doing anything with the request body but we should do
fn parse_request(request: &str) -> Option<(&str, &str)> {
    // Take the request and string and then add 1 to the index to get the next character
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}
