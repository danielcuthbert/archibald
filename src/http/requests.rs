/*
* Archibald: a loyal web server
* Main requests module
* Author: @danielcuthbert
*
* This code serves as the request function.
*/

use super::methods::{Allowedmethods, MethodError};
use super::QueryString;
use crate::http::errors::ParseError;
use core::fmt::Debug;
use std::convert::TryFrom;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{self, Utf8Error};

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
        let request = str::from_utf8(buf)?;
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
