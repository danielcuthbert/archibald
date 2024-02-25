use crate::http::{arch_requests::Requests, arch_response::Response, statuscodes::StatusCode};
use crate::http::methods::Allowedmethods;
use crate::http::validation;
use crate::server::ServerHandler;
use log::{info, warn, debug};
use mime_guess::from_path;
use std::fs;
use std::path::Path;

pub struct ArchibaldHandler {
    static_path: String,
}

impl ArchibaldHandler {
    pub fn new<T: Into<String>>(static_path: T) -> Self {
        let static_path = static_path.into();
        let canonical_static_path = fs::canonicalize(&static_path)
            .expect(&format!("Invalid static path: {}", static_path));

        Self {
            static_path: canonical_static_path.to_string_lossy().to_string(),
        }
    }

    fn read_file(&self, file_path: &str) -> Result<Vec<u8>, StatusCode> {
        let sanitized_path = validation::sanitize_input(file_path);
        debug!("Sanitized path: {}", sanitized_path); // Debugging purpose

        if sanitized_path.contains("..") {
            warn!("Directory traversal attempt detected: {}", sanitized_path);
            return Err(StatusCode::FORBIDDEN);
        }

        let path = format!("{}/{}", self.static_path, sanitized_path);
        debug!("Attempting to read file at path: {}", path); // Debugging purpose

        match fs::canonicalize(&path) {
            Ok(canonical_path) => {
                debug!("Canonical path: {}", canonical_path.display()); // Debugging purpose
                if canonical_path.starts_with(&self.static_path) {
                    match fs::read(canonical_path) {
                        Ok(content) => Ok(content),
                        Err(e) => {
                            warn!("Error reading file: {} ({})", path, e);
                            Err(StatusCode::NotFound)
                        },
                    }
                } else {
                    warn!("Potential security risk detected: {}", file_path);
                    Err(StatusCode::FORBIDDEN)
                }
            },
            Err(e) => {
                warn!("Error resolving file path: {} ({})", path, e);
                Err(StatusCode::NotFound)
            },
        }
    }
}

impl ServerHandler for ArchibaldHandler {
    fn handle_request(&mut self, request: &Requests) -> Response {
        info!("Received request: METHOD {:?}, PATH '{}'", request.method(), request.path());

        match request.method() {
            Allowedmethods::GET => {
                let file_path = if request.path() == "/" {
                    "index.html"
                } else {
                    &request.path()[1..]
                };

                let path = Path::new(&file_path);
                let mime_type = from_path(path).first_or_octet_stream().to_string();

                match self.read_file(file_path) {
                    Ok(content) => {
                        if mime_type.starts_with("text/") || mime_type == "application/javascript" {
                            Response::new(StatusCode::JollyGood, Some(String::from_utf8_lossy(&content).to_string()))
                        } else {
                            Response::new_with_binary(StatusCode::JollyGood, content)
                        }
                    },
                    Err(status) => match status {
                        StatusCode::NotFound => {
                            match self.read_file("404.html") {
                                Ok(content) => Response::new(StatusCode::NotFound, Some(String::from_utf8_lossy(&content).to_string())),
                                Err(_) => Response::new(StatusCode::NotFound, Some("404 Not Found".to_string())),
                            }
                        },
                        StatusCode::FORBIDDEN => Response::new(StatusCode::FORBIDDEN, Some("Access Denied".to_string())),
                        _ => {
                            match self.read_file("500.html") {
                                Ok(content) => Response::new(StatusCode::InternalServerError, Some(String::from_utf8_lossy(&content).to_string())),
                                Err(_) => Response::new(StatusCode::InternalServerError, Some("Internal Server Error".to_string())),
                            }
                        },
                    },
                }
            },
            _ => {
                warn!("Method not allowed: {:?}", request.method());
                Response::new(StatusCode::BadRequest, Some("Method Not Allowed".to_string()))
            },
        }
    }

    fn handle_bad_request(&mut self, _e: &crate::http::errors::ParseError) -> Response {
        warn!("Bad request encountered");
        // Attempt to serve a custom bad request page or return a simple message
        match self.read_file("400.html") {
            Ok(content) => Response::new(StatusCode::BadRequest, Some(String::from_utf8_lossy(&content).to_string())),
            Err(_) => Response::new(StatusCode::BadRequest, Some("Bad Request".to_string())),
        }
    }

    fn handle_request_internal(&mut self, request: &Requests) -> Result<Response, crate::http::errors::ParseError> {
        Ok(self.handle_request(request))
    }
}
