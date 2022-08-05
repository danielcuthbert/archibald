/*
* Archibald: a loyal web server
* This holds the implementation of the handler
* Author: @danielcuthbert
*
*/

// use crate::http::statuscodes;

use crate::http::methods::Allowedmethods;
use crate::http::requests::Request;
use response::Response;

// We make use of a Archibald Handler
use super::http::{methods, requests, response, statuscodes};
// use super::http::response::Response;
// use super::http::Methods;
// use super::http::StatusCode;
use super::http::statuscodes::StatusCode::{JollyGood, NotFound};
use super::server::archibaldserver::ServerHandler;
use std::fs;

// this is the main handler module
// This is for the static_files directory where we serve content from the filesystem.
pub struct ArchibaldHandler {
    static_path: String,
}

// In order to serve the basic index.html page, we need a new handler

impl ArchibaldHandler {
    pub fn new<T: Into<String>>(static_path: T) -> Self {
        Self {
            // a new handler with the static_path
            static_path: static_path.into(),
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.static_path, file_path); // reads the public path
        fs::read_to_string(path).ok() //this will read the file and if there is an error tell us about it. ok() looks at the result and if good converts the value into an option
    }
}

impl ServerHandler for ArchibaldHandler {
    // this handles the request
    fn handle_request(&mut self, request: &Request) -> Response {
        println!("METHOD {:?} PATH '{}'", request.method(), request.path());
        match request.method() {
            // We need to handle the requests depending on what they are. This is where we do that.
            Allowedmethods::GET => match request.path() {
                // If the path is /, we want to return a simple string
                "/" => Response::new(JollyGood, self.read_file("index.html")),
                _ => Response::new(NotFound, Some("Not Found".to_string())),
            },
            // If the query is None, we return a 404 error
            _ => Response::new(NotFound, Some("Not Found".to_string())),
        }
    }

    fn handle_bad_request(&mut self, e: &crate::http::errors::ParseError) -> Response {
        todo!()
    }
}
