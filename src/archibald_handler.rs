use crate::http::errors::ParseError;
use crate::http::methods::Allowedmethods;
use crate::http::validation;
use crate::http::{arch_requests::Requests, arch_response::Response, statuscodes::StatusCode};
use crate::server::ServerHandler;
use log::{debug, info, warn};
use mime_guess::from_path;
use std::fs;
use std::path::Path;
use urlencoding;

pub struct ArchibaldHandler {
    static_path: String,
}

impl ArchibaldHandler {
    pub fn new<T: Into<String>>(static_path: T) -> Self {
        let static_path = static_path.into();
        let canonical_static_path =
            fs::canonicalize(&static_path).expect(&format!("Invalid static path: {}", static_path));

        Self {
            static_path: canonical_static_path.to_string_lossy().to_string(),
        }
    }

    fn read_file(&self, file_path: &str) -> Result<Vec<u8>, StatusCode> {
        let sanitised_path = validation::sanitise_input(file_path);
        debug!("sanitised path: {}", sanitised_path); // Debugging purpose

        if sanitised_path.contains("..") {
            warn!("Directory traversal attempt detected: {}", sanitised_path);
            return Err(StatusCode::Forbidden);
        }

        let path = format!("{}/{}", self.static_path, sanitised_path);
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
                        }
                    }
                } else {
                    warn!("Potential security risk detected: {}", file_path);
                    Err(StatusCode::Forbidden)
                }
            }
            Err(e) => {
                warn!("Error resolving file path: {} ({})", path, e);
                Err(StatusCode::NotFound)
            }
        }
    }
}

impl ServerHandler for ArchibaldHandler {
    fn handle_request(&mut self, request: &Requests) -> Response {
        info!(
            "Received request: METHOD {:?}, PATH '{}'",
            request.method, request.path
        );

        match request.method {
            Allowedmethods::GET => {
                let file_path = if request.path == "/" {
                    "index.html"
                } else {
                    &request.path[1..]
                };

                let full_path = format!("{}/{}", self.static_path, file_path);
                let path = Path::new(&full_path);
                let mime_type = from_path(path).first_or_octet_stream().to_string();

                match self.read_file(file_path) {
                    Ok(content) => {
                        if mime_type.starts_with("text/") || mime_type == "application/javascript" {
                            Response::new(
                                StatusCode::JollyGood,
                                Some(String::from_utf8_lossy(&content).to_string()),
                            )
                            .add_header("Content-Type", &mime_type)
                        } else {
                            Response::new_with_binary(StatusCode::JollyGood, content)
                                .add_header("Content-Type", &mime_type)
                        }
                    }
                    Err(status) => match status {
                        StatusCode::NotFound => Response::new(StatusCode::NotFound, None),
                        StatusCode::Forbidden => {
                            Response::new(StatusCode::Forbidden, Some("Access Denied".to_string()))
                        }
                        _ => Response::new(StatusCode::InternalServerError, None),
                    },
                }
            }
            Allowedmethods::POST => {
                if let Some(body) = &request.body {
                    // Parse form data
                    let lossy_body = String::from_utf8_lossy(body); // Fix: Create a longer-lived value

                    let decoded_data = match urlencoding::decode(&lossy_body) {
                        Ok(data) => data,
                        Err(_) => {
                            // Handle decoding error (e.g., log and return appropriate response)
                            return Response::new(
                                StatusCode::BadRequest,
                                Some("Error processing form data".to_string()),
                            );
                        }
                    };
                    let name_value = decoded_data.split('&').find(|kv| kv.starts_with("name="));

                    // Extract name
                    let name = match name_value {
                        Some(kv) => kv.split('=').nth(1).unwrap(),
                        None => "Unnamed Visitor",
                    };

                    // Create and return response with name
                    return Response::new(
                        StatusCode::JollyGood,
                        Some(format!("Hello, {}!", name)),
                    )
                    .add_header("Content-Type", "text/html"); // Example header
                } else {
                    // Handle empty body case (optional)
                    return Response::new(
                        StatusCode::BadRequest,
                        Some("Empty request body".to_string()),
                    );
                }
            }
            _ => {
                warn!("Method not allowed: {:?}", request.method);
                Response::new(
                    StatusCode::MethodNotAllowed,
                    Some("Method Not Allowed".to_string()),
                )
            }
        }
    }

    fn handle_bad_request(&mut self, _e: &crate::http::errors::ParseError) -> Response {
        warn!("Bad request encountered");
        match self.read_file("400.html") {
            Ok(content) => Response::new(
                StatusCode::BadRequest,
                Some(String::from_utf8_lossy(&content).to_string()),
            ),
            Err(_) => Response::new(StatusCode::BadRequest, Some("Bad Request".to_string())),
        }
    }

    fn handle_request_internal(&mut self, request: &Requests) -> Result<Response, ParseError> {
        Ok(self.handle_request(request))
    }
}


