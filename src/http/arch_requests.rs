use crate::http::methods::Allowedmethods;
use crate::http::errors::ParseError;
use crate::http::Response;  // Import Response
use crate::http::StatusCode;  // Import StatusCode
use mime_guess::from_path;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;

mod validation {
    use crate::http::errors::ParseError;
    use std::path::Path;
    

    pub fn sanitize_input(input: &str) -> String {
        input
            .chars()
            .filter(|&c| c.is_alphanumeric() || c == '/' || c == '.' || c == '-' || c == '_' || c.is_whitespace())
            .collect()
    }

    pub fn validate_input(path: &str) -> Result<(), ParseError> {
        if !Path::new(path).exists() {
            // Convert the std::io::Error into your ParseError type
            return Err(ParseError::NotFound(404));
        }

        Ok(())
    }
}



#[derive(Debug)]
pub struct Requests<'buf> {
    path: &'buf str,
    method: Allowedmethods,
    query_string: Option<&'buf str>,
    file_contents: Option<Vec<u8>>, // needed to serve images
    mime_type: Option<String>, // needed to serve images
}

impl std::fmt::Display for Requests<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl<'buf> Requests<'buf> {
    pub fn new(path: &'buf str, method: Allowedmethods, query_string: Option<&'buf str>) -> Self {
        Requests {
            path,
            method,
            query_string,
            file_contents: None,
            mime_type: None,
        }
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Allowedmethods {
        &self.method
    }
pub fn sanitize_input(input: &str) -> String {
    input
        .chars()
        .filter(|&c| c.is_alphanumeric() || c == '/' || c == '.' || c == '-' || c == '_' || c.is_whitespace())
        .collect()
}
    pub fn query_string(&self) -> Option<&str> {
        self.query_string
    }

    pub fn validate_input(&self) -> Result<(), ParseError> {
        let sanitized_path = validation::sanitize_input(self.path());
        validation::validate_input(&sanitized_path)
    }

    pub fn set_file_contents(&mut self, contents: Vec<u8>) {
        self.file_contents = Some(contents);
    }

    pub fn set_mime_type(&mut self, mime_type: String) {
        self.mime_type = Some(mime_type);
    }

    pub fn handle_request(&mut self, stream: &mut impl Write) -> Result<(), ParseError> {
        self.validate_input()?;
    
        let sanitized_path = validation::sanitize_input(self.path());
        if self.method == Allowedmethods::GET {
            if sanitized_path.starts_with("/static") {
                let file_path = Path::new(&sanitized_path);
                let file_contents = read_binary_file(&file_path)
                    .map_err(|_| ParseError::NotFound(404))?;
                let mime_type = get_mime_type(&file_path);
    
                if mime_type.starts_with("image/") {
                    let file_contents = read_binary_file(&file_path)
                        .map_err(|_| ParseError::NotFound(404))?;

                    let response = Response::new_with_binary(StatusCode::JollyGood, file_contents);
                    response.send(stream)?;
                } else {
                    let file_contents = read_text_file(&file_path)
                        .map_err(|_| ParseError::NotFound(404))?;

                    let response = Response::new(StatusCode::JollyGood, Some(file_contents));
                    response.send(stream)?;
                }
            } else {
                // Handle other paths
            }
        }
    
        Ok(())
    }
}


impl<'buf> TryFrom<&'buf [u8]> for Requests<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Requests<'buf>, Self::Error> {
        let request_str = std::str::from_utf8(buf)?;

        let parts: Vec<&str> = request_str.split_whitespace().collect();
        if parts.len() < 3 {
            return Err(ParseError::InvalidRequest);
        }

        let method = parts[0];
        let path = parts[1];
        let protocol = parts[2];

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Allowedmethods = method.parse()?;
        let request = Requests::new(path, method, None);

        Ok(request)
    }
}

fn read_binary_file(file_path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn get_mime_type(file_path: &Path) -> String {
    from_path(file_path).first_or_octet_stream().to_string()
}

fn parse_request(request: &str) -> Option<(&str, &str)> {
    request.split_once(' ').or_else(|| request.split_once('\r'))
}

fn read_text_file(file_path: &Path) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
