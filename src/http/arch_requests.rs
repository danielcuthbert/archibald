use crate::http::errors::ParseError;
use crate::http::methods::Allowedmethods;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::Path;
// use crate::http::Response;
// use crate::http::StatusCode;
use mime_guess::from_path;

// Validation module remains as you defined.
mod validation {
    use super::ParseError; // Adjust based on actual module structure
    use std::fs;
    use std::io::ErrorKind;
    use std::path::Path;

    pub fn sanitise_input(input: &str) -> String {
        input
            .chars()
            .filter(|&c| {
                c.is_alphanumeric()
                    || c == '/'
                    || c == '.'
                    || c == '-'
                    || c == '_'
                    || c.is_whitespace()
            })
            .collect()
    }

    pub fn validate_input(input: &str) -> Result<(), ParseError> {
        let sanitised_path = sanitise_input(input);

        if !Path::new(&sanitised_path).exists() {
            let file_error = fs::metadata(&sanitised_path).err().unwrap(); // Consider handling this unwrap more safely

            match file_error.kind() {
                ErrorKind::NotFound => return Err(ParseError::NotFound(404)),
                ErrorKind::PermissionDenied => {
                    return Err(ParseError::IOError(format!(
                        "File permission error: {}",
                        file_error
                    )))
                }
                _ => return Err(ParseError::IOError("Unknown file error".into())),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Requests<'buf> {
    pub path: &'buf str,
    pub method: Allowedmethods,
    pub query_string: Option<&'buf str>,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl<'buf> TryFrom<&'buf [u8]> for Requests<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request_str = std::str::from_utf8(buf).map_err(|_| ParseError::InvalidRequest)?;

        let (request_line, rest) = request_str
            .split_once("\r\n")
            .ok_or(ParseError::InvalidRequest)?;
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() != 3 {
            return Err(ParseError::InvalidRequest);
        }

        let method = parts[0]
            .parse::<Allowedmethods>()
            .map_err(|_| ParseError::InvalidMethod)?;
        let full_path = parts[1];
        let (path, query_string) = full_path.split_once('?').unwrap_or((full_path, ""));

        let mut headers = HashMap::new();
        let (raw_headers, body) = rest.split_once("\r\n\r\n").unwrap_or((rest, ""));
        for line in raw_headers.lines() {
            let (key, value) = line.split_once(": ").ok_or(ParseError::InvalidHeader)?;
            headers.insert(key.to_string(), value.to_string());
        }

        Ok(Requests {
            path,
            method,
            query_string: if query_string.is_empty() {
                None
            } else {
                Some(query_string)
            },
            headers,
            body: Some(body.as_bytes().to_vec()),
        })
    }
}

impl<'buf> Requests<'buf> {
    // Additional methods related to Requests can be placed here.
    // For instance, methods to access the headers, body, etc., if needed beyond the struct field access.
}

// If you have additional utility functions like reading files, include them here.
fn read_binary_file(file_path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn get_mime_type(file_path: &Path) -> String {
    from_path(file_path).first_or_octet_stream().to_string()
}
