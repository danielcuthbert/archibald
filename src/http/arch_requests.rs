use super::methods::{Allowedmethods, MethodError};
use super::QueryString;
use crate::http::errors::ParseError;
use core::fmt::Debug;
use std::convert::TryFrom;
use std::path;


mod validation {
    use std::error::Error;

    use crate::http::{methods::Allowedmethods, validation, ParseError};
    use regex::Regex;
    
    pub fn sanitize_input(input: &str) -> Result<(), Box<dyn Error>> {
        let mut sanitized_input = String::new();
        for character in input.chars() {
            if !Regex::new(r"[^\w\s]").unwrap().is_match(&character.to_string()) {
                sanitized_input.push(character);
            }
        }
    
        Ok(sanitized_input)
    }

    pub fn validate_input(path: &str) -> Result<(), Box<dyn Error>> {
        let sanitized_path = sanitize_input(path)?;
    
        validation::validate_input(&sanitized_path)
    }
    
}

#[derive(Debug)]
pub struct Requests<'buf> {
    path: &'buf str,
    reqpath: &str,
    method: Allowedmethods,
}

impl std::fmt::Display for Requests<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

// We need to send back a response to the client
// This will be a byte array to a string
// https://doc.rust-lang.org/std/convert/trait.From.html
// Note: This trait must not fail. If the conversion can fail, use TryFrom.

// example request
// GET /name?first=Daniel&last=Cuthbert HTTP/1.1
// In order to get all of the request, we have to parse it word by word somehow

impl Requests<'_> {
    /// This allows us to create a new request. It makes use of Allowedmethods to check if the method is valid.
    pub fn path(&self) -> &str {
        &self.path()
    }
    pub fn method(&self) -> &Allowedmethods {
        &self.method
    }

    pub fn validate_input(&self) -> Result<(), ParseError> {
        let sanitized_path = validation::sanitize_input(self.path());

        validation::validate_input(&sanitized_path)
    }
}

// Handle utf-8 errors
// this bit is frustrating as hell and hurting me more than it should.
// I know I need to return something but I don't know what

impl<'buf> TryFrom<&'buf [u8]> for Requests<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Requests<'buf>, Self::Error> {
        let request = std::str::from_utf8(buf)?;

        let (_method, mut request) = parse_request(request).ok_or(ParseError::InvalidRequest)?;
        let (_protocol, _) = parse_request(request).ok_or(ParseError::InvalidRequest)?;

        if _protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Allowedmethods = _method.parse()?;

        let mut query_string = None;
        if let Some(i) = request.find('?') {
            query_string = Some(QueryString::from(&request[i + 1..]));
            request = &request[..i];
        }

        let request = Requests {
            reqpath: request,
            query_string,
            method,
            path: todo!(),
            
        };

        request.validate_input()?;

        Ok(request)
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
    
