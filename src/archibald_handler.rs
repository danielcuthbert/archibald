/**
* Archibald: a loyal web server
* This holds the implementation of the handler
* Author: @danielcuthbert
*
**/
// The ArchibaldHandler struct holds the path to the directory containing the static files. The new method is used to create a new instance of the ArchibaldHandler struct with the given static path.

// The read_file method is used to read a file from the static directory. It takes a file path as an argument and returns the contents of the file as a String.
// The method first constructs the absolute path to the file by concatenating the static path and the file path. It then checks if the absolute path is valid and starts with the static path.
// If the path is valid, it reads the contents of the file and returns it as a String. If the path is not valid, it returns None.

// The handle_request method is used to handle incoming HTTP requests. It takes a Request object as an argument and returns a Response object.
// The method first logs the HTTP method and path of the request. If the request method is GET, it checks the request path and returns the appropriate response.
// If the request path is /, it returns the contents of the index.html file. If the request path is not /, it reads the file with the given path and returns its contents.
// If the request method is not GET, it returns a 404 Not Found response.

// The handle_bad_request method is not implemented and is currently a stub. It takes a ParseError object as an argument and returns a Response object.

// use crate::http::statuscodes;
use crate::http::methods::Allowedmethods;
use crate::http::requests::Request;
use crate::http::statuscodes::StatusCode;
use log::info;
use response::Response;

// We make use of a Archibald Handler
use super::http::{methods, requests, response, statuscodes};
// use super::http::response::Response;
// use super::http::Methods;
// use super::http::StatusCode;
use super::http::statuscodes::StatusCode::{JollyGood, NotFound};
use super::server::archibaldserver::ServerHandler;
use std::fs;

/// this is the main handler module
/// This is for the static_files directory where we serve content from the filesystem.
pub struct ArchibaldHandler {
    static_path: String,
}

/// In order to serve the basic index.html page, we need a new handler

impl ArchibaldHandler {
    pub fn new<T: Into<String>>(static_path: T) -> Self {
        Self {
            // a new handler with the static_path
            static_path: static_path.into(),
        }
    }

    /// This is where we could introduce an ugly vulnerability called directory traversal if we do not validate properly.
    /// What we need to do is check that the path is the absolute path to the static_files directory.
    /// the file_path is provided by the request and is possibly malicious.
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.static_path, file_path); // reads the public path. This is actually a vulnerability if we dont check the path
                                                                  // we can use the fs::canonicalize function to get the absolute path and remove the .. ../
        match fs::canonicalize(path) {
            Ok(path) => {
                /// if the path is valid (exists) and is the static_path we defined, then we can read the file
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
    /// This handles the request
    fn handle_request(&mut self, request: &Request) -> Response {
        info!("METHOD {:?} PATH '{}'", request.method(), request.path());
        match request.method() {
            // If a GET request is made, we need to check the path and return the appropriate response
            Allowedmethods::GET => match request.path() {
                "/" => Response::new(JollyGood, self.read_file("index.html")),
                // This is the default case where if nothing matches, we return a 404
                _ => Response::new(StatusCode::NotFound, None),

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
