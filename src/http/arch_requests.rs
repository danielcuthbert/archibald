use crate::http::errors::ParseError;
use crate::http::methods::Allowedmethods;
use crate::http::Response; // Import Response
use crate::http::StatusCode; // Import StatusCode
use mime_guess::from_path;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod validation {
    use crate::http::errors::ParseError;
    use std::{fs, io::ErrorKind, path::Path};

    pub fn sanitise_input(input: &str) -> String {
        input.chars().filter(|&c| c.is_alphanumeric() || c == '/' || c == '.' || c == '-' || c == '_' || c.is_whitespace()).collect()
    }

    pub fn validate_input(input: &str) -> Result<(), ParseError> {
        let sanitised_path = sanitise_input(input);

        if !Path::new(&sanitised_path).exists() {
            let file_error = fs::metadata(&sanitised_path).err().unwrap();

            match file_error.kind() {
                ErrorKind::NotFound => return Err(ParseError::NotFound(404)),
                ErrorKind::PermissionDenied => {
                    return Err(ParseError::IOError(format!("File permission error: {}", file_error.to_string())))
                }
                _ => return Err(ParseError::IOError(file_error.to_string())),
            }
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
    mime_type: Option<String>,      // needed to serve images
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

    pub fn query_string(&self) -> Option<&str> {
        self.query_string
    }

    pub fn set_file_contents(&mut self, contents: Vec<u8>) {
        self.file_contents = Some(contents);
    }

    pub fn set_mime_type(&mut self, mime_type: String) {
        self.mime_type = Some(mime_type);
    }

    pub fn handle_request(&mut self, stream: &mut impl Write) -> Result<(), ParseError> {
        let sanitised_path = validation::sanitise_input(self.path());

        if self.method == Allowedmethods::GET {
            let file_path = Path::new(&sanitised_path);

            match read_binary_file(file_path) {
                Ok(file_contents) => {
                    let mime_type = get_mime_type(file_path);

                    let response = if mime_type.starts_with("image/") {
                        Response::new_with_binary(StatusCode::JollyGood, file_contents)
                    } else {
                        Response::new(StatusCode::JollyGood, Some(String::from_utf8_lossy(&file_contents).to_string()))
                            .add_header("Content-Type", &mime_type)
                    };
                    response.send(stream)?;
                },
                Err(_) => {
                    Response::send_error(StatusCode::NotFound).send(stream)?;
                }
            }
        } else {
            // Handle other paths or methods
            Response::send_error(StatusCode::BadRequest).send(stream)?;
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

pub fn get_mime_type(file_path: &Path) -> String {
    from_path(file_path).first_or_octet_stream().to_string()
}

fn read_text_file(file_path: &Path) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
