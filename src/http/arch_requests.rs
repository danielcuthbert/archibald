use super::methods::Allowedmethods;
use crate::http::errors::ParseError;




mod validation {
    use crate::http::errors::ParseError;
    use std::path::Path;
    // use std::error::Error;

    pub fn sanitize_input(input: &str) -> String {
        input.chars()
             .filter(|&c| c.is_alphanumeric() || c.is_whitespace())
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
}

impl std::fmt::Display for Requests<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl<'buf> Requests<'buf> {
    pub fn new(path: &'buf str, method: Allowedmethods, query_string: Option<&'buf str>) -> Self {
        Requests { path, method, query_string }
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn method(&self) -> &Allowedmethods {
        &self.method
    }

    pub fn validate_input(&self) -> Result<(), ParseError> {
        let sanitized_path = validation::sanitize_input(self.path());
        validation::validate_input(&sanitized_path)
    }

    pub fn query_string(&self) -> Option<&str> {
        self.query_string
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
        // You need to extract the query string from the path here, if it exists.
        // For now, I'm assuming the whole path is being passed.
        let request = Requests::new(path, method, None); // Adjust according to how query_string should be handled

        Ok(request)
    }
}


fn parse_request(request: &str) -> Option<(&str, &str)> {
    request.split_once(' ').or_else(|| request.split_once('\r'))
}

