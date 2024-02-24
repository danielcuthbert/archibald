use crate::http::{arch_requests::Requests, arch_response};

use crate::http::methods::Allowedmethods;
use crate::http::validation;
use crate::server::ServerHandler;
use arch_response::Response;
use log::{info, warn};
use mime_guess::from_path;
use std::fs;
use std::path::Path;

use super::http::statuscodes::StatusCode::{JollyGood, NotFound};

pub struct ArchibaldHandler {
    static_path: String,
}

impl ArchibaldHandler {
    pub fn new<T: Into<String>>(static_path: T) -> Self {
        let static_path = static_path.into();
        let canonical_static_path = fs::canonicalize(&static_path)
            .unwrap_or_else(|_| panic!("Invalid static path: {}", static_path));

        Self {
            static_path: canonical_static_path.to_string_lossy().to_string(),
        }
    }

    fn read_file(&self, file_path: &str) -> Option<Vec<u8>> {
        let sanitized_path = validation::sanitize_input(file_path);
        info!("Sanitized path: {}", sanitized_path);

        if sanitized_path.contains("..") {
            warn!("Directory traversal attempt detected: {}", sanitized_path);
            return None;
        }

        let path = format!("{}/{}", self.static_path, sanitized_path);
        info!("Attempting to read file at path: {}", path);

        match fs::canonicalize(&path) {
            Ok(canonical_path) => {
                info!("Canonical path: {}", canonical_path.display());
                if canonical_path.starts_with(&self.static_path) {
                    fs::read(canonical_path).ok()
                } else {
                    warn!("Potential security risk detected: {}", file_path);
                    None
                }
            }
            Err(e) => {
                warn!("Error reading file: {}", e);
                None
            }
        }
    }
}

impl ServerHandler for ArchibaldHandler {
    fn handle_request(&mut self, request: &Requests) -> Response {
        info!(
            "Received request: METHOD {:?}, PATH '{}'",
            request.method(),
            request.path()
        );

        match request.method() {
            Allowedmethods::GET => {
                let file_path = if request.path() == "/" {
                    "index.html" // Serve index.html if root is requested
                } else {
                    &request.path()[1..] // Serve file directly
                };

                let path = Path::new(&file_path);
                let mime_type = from_path(path).first_or_octet_stream().to_string();

                match self.read_file(file_path) {
                    Some(content) => {
                        if mime_type.starts_with("text/") || mime_type == "application/javascript" {
                            // Handle as text
                            Response::new(
                                JollyGood,
                                Some(String::from_utf8_lossy(&content).to_string()),
                            )
                        } else {
                            // Handle as binary
                            Response::new_with_binary(JollyGood, content)
                        }
                    }
                    None => {
                        warn!("File not found or access denied: {}", file_path);
                        Response::new(NotFound, Some("Access Denied".to_string()))
                    }
                }
            }
            _ => {
                warn!("Method not allowed: {:?}", request.method());
                Response::new(NotFound, Some("Method Not Allowed".to_string()))
            }
        }
    }

    fn handle_bad_request(&mut self, _e: &crate::http::errors::ParseError) -> Response {
        warn!("Bad request encountered");
        Response::new(NotFound, Some("Bad Request".to_string()))
    }

    fn handle_request_internal(
        &mut self,
        request: &Requests,
    ) -> Result<Response, crate::http::errors::ParseError> {
        Ok(self.handle_request(request))
    }
}
