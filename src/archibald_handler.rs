// src/archibald_handlers.rs

use crate::http::errors::ParseError;
use crate::http::methods::AllowedMethods;
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
        // Sanitize input path
        let sanitized_path = file_path.trim_start_matches('/');
        debug!("Sanitized path: {}", sanitized_path); // Debugging purpose

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
            AllowedMethods::GET => {
                let file_path = if request.path == "/" {
                    "index.html"
                } else {
                    &request.path[1..]
                };

                let path = Path::new(file_path);
                let mime_type = from_path(path).first_or_octet_stream().to_string();

                match self.read_file(file_path) {
                    Ok(content) => {
                        if mime_type.starts_with("text/") || mime_type == "application/javascript" {
                            Response::new_with_text(
                                StatusCode::OK,
                                &String::from_utf8_lossy(&content),
                                &mime_type,
                            )
                            .add_header("Content-Type", &mime_type)
                        } else {
                            Response::new(StatusCode::OK, content, &mime_type)
                                .add_header("Content-Type", &mime_type)
                        }
                    }
                    Err(status) => match status {
                        StatusCode::NotFound => {
                            Response::new_with_text(StatusCode::NotFound, "Not Found", "text/plain")
                        }
                        StatusCode::Forbidden => Response::new_with_text(
                            StatusCode::Forbidden,
                            "Access Denied",
                            "text/plain",
                        ),
                        _ => Response::new_with_text(
                            StatusCode::InternalServerError,
                            "Internal Server Error",
                            "text/plain",
                        ),
                    },
                }
            }
            AllowedMethods::POST => {
                if let Some(body) = &request.body {
                    // Parse form data
                    let lossy_body = String::from_utf8_lossy(body);

                    let decoded_data = match urlencoding::decode(&lossy_body) {
                        Ok(data) => data,
                        Err(_) => {
                            // Handle decoding error
                            return Response::new_with_text(
                                StatusCode::BadRequest,
                                "Error processing form data",
                                "text/plain",
                            );
                        }
                    };

                    let name_value = decoded_data.split('&').find(|kv| kv.starts_with("name="));

                    // Extract name
                    let name = match name_value {
                        Some(kv) => kv.split('=').nth(1).unwrap_or("Unnamed Visitor"),
                        None => "Unnamed Visitor",
                    };

                    // Create and return response with name
                    Response::new_with_text(
                        StatusCode::OK,
                        &format!("Hello, {}!", name),
                        "text/html",
                    )
                    .add_header("Content-Type", "text/html") // Example header
                } else {
                    // Handle empty body case
                    Response::new_with_text(
                        StatusCode::BadRequest,
                        "Empty request body",
                        "text/plain",
                    )
                }
            }
            _ => {
                warn!("Method not allowed: {:?}", request.method);
                Response::new_with_text(
                    StatusCode::MethodNotAllowed,
                    "Method Not Allowed",
                    "text/plain",
                )
            }
        }
    }

    fn handle_bad_request(&mut self, _e: &crate::http::errors::ParseError) -> Response {
        warn!("Bad request encountered");
        match self.read_file("400.html") {
            Ok(content) => Response::new(StatusCode::BadRequest, content, "text/html"),
            Err(_) => Response::new_with_text(StatusCode::BadRequest, "Bad Request", "text/plain"),
        }
    }
}
