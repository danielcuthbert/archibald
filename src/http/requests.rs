/*
* Archibald: a loyal web server
* Main requests module
* Author: @danielcuthbert
*
*/

use crate::http::methods::Allowedmethods;
use std::convert::TryFrom;
use crate::http::requests;
use crate::http::errors::ParseError;
use std::str;


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

// impl Request {
//     // If we receive an invalid request, we need to return a 400 Bad Request
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String>{}
// }

// We need to create another implementation block for the TryFrom trait

// Convert a byte slice to a string
impl TryFrom<&[u8]> for Request {
    type Error = String;
    // this is the actual function that does stuff. copied from the docs. 
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // We need to match on the results, so use match
        match str::from_utf8(buf) {
Ok(request) => {}
Err(_) => {
    // We need to return an error if the conversion fails
    return Err(ParseError::InvalidEncoding);
        
        unimplemented!()
    }
}
