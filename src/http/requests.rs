/*
* Archibald: a loyal web server
* Main requests module
* Author: @danielcuthbert
*
* This code serves as the request function.
* The code above is a Rust module that defines a Request struct and an implementation for it. The Request struct holds information about an HTTP request, including the request path, query string, and HTTP method. The Request struct also has methods to access these fields.

* The Request struct is defined with a lifetime parameter 'buf, which is used to specify the lifetime of the string slice that holds the request path. This is necessary because the Request struct holds a reference to the request path, which is a slice of the original request buffer.

* The Request struct also has an implementation of the TryFrom trait, which allows it to be created from a byte slice. The TryFrom implementation parses the byte slice into a string, then parses the string into the various fields of the Request struct.

* The parse_request function is a helper function used by the TryFrom implementation to parse the request string. It takes a string slice and returns an Option containing a tuple of two string slices: the first slice is the next word in the request string, and the second slice is the remainder of the request string.
*/
use super::methods::{Allowedmethods, MethodError};
use super::QueryString;
use crate::http::errors::ParseError;
use core::fmt::Debug;
use std::convert::TryFrom;

// use std::error::Error;
// use std::fmt::{Display, Formatter};
// use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString>,
    method: Allowedmethods,
}

// We need to send back a response to the client
// This will be a byte array to a string
// https://doc.rust-lang.org/std/convert/trait.From.html
// Note: This trait must not fail. If the conversion can fail, use TryFrom.

// example request
// GET /name?first=Daniel&last=Cuthbert HTTP/1.1
// In order to get all of the request, we have to parse it word by word somehow

impl Request<'_> {
    /// This allows us to create a new request. It makes use of Allowedmethods to check if the method is valid.
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Allowedmethods {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// Handle utf-8 errors
// this bit is frustrating as hell and hurting me more than it should.
// I know I need to return something but I don't know what

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = std::str::from_utf8(buf)?;

        let (_method, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (_protocol, _) = parse_request(request).ok_or(ParseError::InvalidRequest)?;

        if _protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Allowedmethods = _method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn parse_request(request: &str) -> Option<(&str, &str)> {
    // Take the request and string and then add 1 to the index to get the next character
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}
