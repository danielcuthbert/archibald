use crate::http::arch_requests::Requests;
use crate::http::methods::Allowedmethods;
use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ValidationParseError {
    InvalidMethod,
    MaliciousPath,
    MaliciousQueryString,
    InvalidRegex, // Added to handle regex errors
}

impl Display for ValidationParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            ValidationParseError::InvalidMethod => write!(f, "Invalid method"),
            ValidationParseError::MaliciousPath => write!(f, "Malicious path detected"),
            ValidationParseError::MaliciousQueryString => {
                write!(f, "Malicious query string detected")
            }
            ValidationParseError::InvalidRegex => write!(f, "Invalid regex pattern"),
        }
    }
}

impl Error for ValidationParseError {}

pub fn sanitize_input(input: &str) -> String {
    let re = Regex::new(r"[^\w\s./]").expect("Invalid regex pattern"); // Allow dots and slashes
    re.replace_all(input, "").to_string()
}

pub fn validate_input(request: &Requests) -> Result<(), ValidationParseError> {
    let method = request.method();
    let path = request.path();
    let query_string = request.query_string();

    if !Allowedmethods::is_valid(method) {
        return Err(ValidationParseError::InvalidMethod);
    }

    if path.contains('\'') || path.contains('\"') || path.contains(';') {
        return Err(ValidationParseError::MaliciousPath);
    }

    if let Some(query_string) = query_string {
        for pair in query_string.split('&') {
            let parts: Vec<&str> = pair.split('=').collect();
            if let Some(key) = parts.get(0) {
                if key.contains('\'') || key.contains('\"') || key.contains(';') {
                    return Err(ValidationParseError::MaliciousQueryString);
                }
            }
        }
    }

    Ok(())
}
