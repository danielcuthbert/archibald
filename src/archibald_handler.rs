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
use crate::http::arch_requests::Requests;
use crate::http::statuscodes::StatusCode;
use crate::server::ServerHandler;
use log::info;
use arch_response::Response;
use std::path::PathBuf;

// We make use of a Archibald Handler
use super::http::{arch_response};
// use super::http::response::Response;
// use super::http::Methods;
// use super::http::StatusCode;
use super::http::statuscodes::StatusCode::{JollyGood, NotFound};

use std::fs;

/// this is the main handler module
/// This is for the static_files directory where we serve content from the filesystem.
pub struct ArchibaldHandler {
    static_path: String,
}

/// In order to serve the basic index.html page, we need a new handler

impl ArchibaldHandler {
    pub fn new<T: Into<String>>(static_path: T) -> Self {
        let static_path = static_path.into();
        let canonical_static_path = fs::canonicalize(&static_path)
            .unwrap_or_else(|_| panic!("Invalid static path: {}", static_path));

        Self {
            static_path: canonical_static_path.to_string_lossy().to_string(),
        }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        if file_path.contains("..") {
            println!("Directory traversal attack attempt detected: {}", file_path);
            return None;
        }

        let path = format!("{}/{}", self.static_path, file_path);

        match fs::canonicalize(PathBuf::from(path)) {
            Ok(canonical_path) => {
                if canonical_path.starts_with(&self.static_path) {
                    fs::read_to_string(canonical_path).ok()
                } else {
                    println!("Potential directory traversal attack: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl ServerHandler for ArchibaldHandler {
    fn handle_request(&mut self, request: &Requests) -> Response {
        info!("METHOD {:?} PATH '{}'", request.method(), request.path());

        match request.method() {
            Allowedmethods::GET => {
                let path = if request.path() == "/" {
                    "index.html"  // Serve index.html if root is requested
                } else {
                    &request.path()[1..]  // Serve file directly
                };

                match self.read_file(path) {
                    Some(content) => Response::new(JollyGood, Some(content)),
                    None => Response::new(NotFound, Some("Access Denied".to_string())),
                }
            },
            _ => Response::new(NotFound, Some("Method Not Allowed".to_string())),
        }
    }

    fn handle_bad_request(&mut self, _e: &crate::http::errors::ParseError) -> Response {
        Response::new(NotFound, Some("Bad Request".to_string()))
    }

    fn handle_request_internal(&mut self, request: &Requests) -> Result<Response, crate::http::errors::ParseError> {
        Ok(self.handle_request(request))
    }
}
