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

    // This is where we could introduce an ugly vulnerability called directory traversal if we do not validate properly.
    // What we need to do is check that the path is the absolute path to the static_files directory.
    // the file_path is provided by the request and is possibly malicious.
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.static_path, file_path); // reads the public path. This is actually a vulnerability if we dont check the path
                                                                  // we can use the fs::canonicalize function to get the absolute path and remove the .. ../
        match fs::canonicalize(path) {
            Ok(path) => {
                // if the path is valid (exists) and is the static_path we defined, then we can read the file
                if path.starts_with(&self.static_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!(
                        "I say old boy, what are you doing?. This looks like an attack: {}",
                        file_path
                    );
                    return None;
                }
            }
            Err(_) => None,
        }
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

                path => match self.read_file(path) {
                    Some(content) => Response::new(JollyGood, Some(content)),
                    None => Response::new(NotFound, None),
                },
            },
            // When we dont have a mapping, we return a 404
            _ => Response::new(NotFound, Some("Not Found".to_string())),
        }
    }

    fn handle_bad_request(&mut self, e: &crate::http::errors::ParseError) -> Response {
        todo!()
    }
}
