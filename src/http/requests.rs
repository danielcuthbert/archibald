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

// example request
// GET /name?first=Daniel&last=Cuthbert HTTP/1.1 
// In order to get all of the request, we have to parse it word by word somehow 

// Convert a byte slice to a string
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // this is the actual function that does stuff. copied from the docs. 
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // We need to match on the results, so use match
let request = str::from_utf8(buf)?;
match parse_request(request) {
    Ok(req) => Ok(req),
    Err(e) => Err(e),
}
    }
}
// basically accepts the request, which is a string slice 
// so get 'method', 'path', 'query', 'body' etc 
// we need a loop here to get all of the request
// I don't think this is the best way to do this at all and probably breaks shit


fn parse_request(request: &str) -> Option<(&str, &str)>  {
    let mut iterate = request.chars();
    loop {
        let mut current = iterate.next();
        if current == None {
            break;
        }
        let mut next = iterate.next();
        if next == None {
            break;
        }
        if current == Some(' ') && next == Some(' ') {
            break;
        }

        for d in request.chars() {
            if d == ' ' {
                break;
            }
        }
    }
}
