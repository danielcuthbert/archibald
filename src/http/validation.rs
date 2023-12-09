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
    println!("Original path: {}", input); // Log the original path

    let re = Regex::new(r"[^\w\s./-]").expect("Invalid regex pattern");
    let sanitized = re.replace_all(input, "").to_string();

    // Replace any '..' sequences to prevent directory traversal attacks
    let final_sanitized = sanitized.replace("../", "").replace("/../", "");

    println!("Sanitized path: {}", final_sanitized); // Log the sanitized path

    final_sanitized
}

pub fn validate_input(request: &Requests) -> Result<(), ValidationParseError> {
    let method = request.method();
    let path = request.path();
    let query_string = request.query_string();

    if !Allowedmethods::is_valid(method) {
        return Err(ValidationParseError::InvalidMethod);
    }

    // Check for any other potentially dangerous characters or sequences in the path
    if path.contains('\'') || path.contains('\"') || path.contains(';') || path.contains("..") {
        return Err(ValidationParseError::MaliciousPath);
    }

    if let Some(query_string) = query_string {
        for pair in query_string.split('&') {
            let parts: Vec<&str> = pair.split('=').collect();
            // Check both the key and the value for potentially dangerous characters
            for part in parts {
                if part.contains('\'') || part.contains('\"') || part.contains(';') {
                    return Err(ValidationParseError::MaliciousQueryString);
                }
            }
        }
    }

    Ok(())
}
