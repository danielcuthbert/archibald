// src/http/arch_requests.rs

use crate::http::errors::ParseError;
use crate::http::methods::AllowedMethods;
use crate::http::arch_response::Response;
use crate::http::statuscodes::StatusCode;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Component, Path, PathBuf};
use log::{debug, warn};
use mime_guess::from_path;
use percent_encoding::percent_decode_str;

mod validation {
    use super::ParseError;
    use std::path::{Component, Path, PathBuf};
    use log::debug;
    use percent_encoding::percent_decode_str;
    use urlencoding::decode;

    /// Validates and sanitizes the input path to prevent path traversal attacks.
    pub fn validate_input(input: &str) -> Result<PathBuf, ParseError> {
        debug!("Validating input path: {}", input);
    
        // URL-decode the input path
        let decoded_input = percent_decode_str(input)
            .decode_utf8()
            .map_err(|_| ParseError::InvalidEncoding)?;
        let decoded_input = decoded_input.as_ref();
    
        debug!("Decoded input path: {}", decoded_input);
    
        // Start with an empty path
        let mut path = PathBuf::new();
    
        // Sanitize the input path by filtering out dangerous components
        for component in Path::new(decoded_input).components() {
            match component {
                Component::Normal(name) => {
                    // Append valid path components
                    path.push(name);
                }
                Component::RootDir => {
                    // Skip the root directory component to create a relative path
                    // This allows paths starting with '/' without including it in the PathBuf
                    continue;
                }
                Component::CurDir => {
                    // Ignore the current directory component '.'
                    continue;
                }
                Component::ParentDir => {
                    // Reject attempts to navigate to parent directories
                    debug!("Invalid path component (ParentDir): {:?}", component);
                    return Err(ParseError::InvalidPath);
                }
                _ => {
                    // Reject any other components (e.g., prefix, verbatim)
                    debug!("Invalid path component: {:?}", component);
                    return Err(ParseError::InvalidPath);
                }
            }
        }
    
        // If the path is empty after processing, default to "index.html"
        if path.as_os_str().is_empty() {
            path.push("index.html");
        }
    
        debug!("Sanitized path: {:?}", path);
    
        Ok(path)
    }
}

#[derive(Debug)]
pub struct Requests<'buf> {
    pub path: &'buf str,
    pub method: AllowedMethods,
    pub query_string: Option<&'buf str>,
    pub headers: HashMap<&'buf str, &'buf str>,
    pub body: Option<Vec<u8>>, // Changed to own the body data
}

impl<'buf> TryFrom<&'buf [u8]> for Requests<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // Convert the byte buffer to a string
        let request_str = std::str::from_utf8(buf).map_err(|e| {
            warn!("Failed to parse request as UTF-8: {}", e);
            ParseError::InvalidEncoding
        })?;

        // Log the request string
        debug!("Request string: {}", request_str);

        // Split request into lines using CRLF
        // The split("\r\n") method accurately splits the HTTP request into lines at each CRLF sequence, 
        // which is the standard line ending in HTTP requests. 
        // This avoids any trailing \r characters in the lines, which can interfere with parsing headers and request lines.
        let mut lines = request_str.split("\r\n");

        // Parse the request line
        let request_line = lines.next().ok_or(ParseError::InvalidRequest)?;
        let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ParseError::InvalidRequest);
        }

        // Parse method
        let method = parts[0]
            .parse::<AllowedMethods>()
            .map_err(|_| ParseError::InvalidMethod)?;

        // Parse path and query string
        let full_path = parts[1];
        let (path, query_string) = full_path.split_once('?').unwrap_or((full_path, ""));

        // Validate the path
        validation::validate_input(path)?;

        // Parse headers
        let mut headers = HashMap::new();
        // After parsing headers
        debug!("Parsed headers: {:?}", headers);

        for line in lines.by_ref() {
            if line.is_empty() {
                break; // End of headers
            }
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                debug!("Parsed header: '{}: {}'", key, value);
                headers.insert(key, value);
            } else {
                warn!("Invalid header format: {}", line);
                return Err(ParseError::InvalidHeader);
            }
        }
        
        

        // Collect the body if present
        let body = lines.collect::<Vec<&str>>().join("\n");
        let body = if body.is_empty() { None } else { Some(body.as_bytes().to_vec()) };

        Ok(Requests {
            path,
            method,
            query_string: if query_string.is_empty() {
                None
            } else {
                Some(query_string)
            },
            headers,
            body,
        })
    }
}


impl<'buf> Requests<'buf> {
    // Additional methods can be added here
}

fn read_binary_file(file_path: &Path) -> std::io::Result<Vec<u8>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn get_mime_type(file_path: &Path) -> String {
    from_path(file_path).first_or_octet_stream().to_string()
}

// Example of how to handle a request using the above functions
pub fn handle_request(request_bytes: &[u8], stream: &mut impl Write) -> Result<(), ParseError> {
    let request = Requests::try_from(request_bytes)?;

    match request.method {
        AllowedMethods::GET => {
            let file_path = validation::validate_input(request.path)?;
            let content = read_binary_file(&file_path)?;
            let mime_type = get_mime_type(&file_path);
            let response = Response::new(StatusCode::OK, content, &mime_type);
            response.send(stream)?;
            Ok(())
        }
        _ => {
            let response = Response::new(
                StatusCode::MethodNotAllowed,
                Vec::new(),
                "text/plain",
            );
            response.send(stream)?;
            Ok(())
        }
    }
}
