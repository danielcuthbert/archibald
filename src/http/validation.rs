use crate::http::arch_requests::Requests;
use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};

// Simplified validation error enum
#[derive(Debug)]
pub enum ValidationParseError {
    InvalidMethod,
    MalformedPath,
    VulnerablePath,
    VulnerableQueryString,
}

impl Display for ValidationParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            ValidationParseError::InvalidMethod => write!(f, "Invalid method"),
            ValidationParseError::VulnerablePath => write!(f, "Malicious path detected"),
            ValidationParseError::VulnerableQueryString => {
                write!(f, "Malicious query string detected")
            }
            ValidationParseError::MalformedPath => write!(f, "Malformed path"),
        }
    }
}

impl Error for ValidationParseError {}

// Sanitization function using regular expressions
pub fn sanitize_input(input: &str) -> String {
    println!("Original path: {}", input);

    let re = Regex::new(r"[^\w\s./-]").expect("Invalid regex pattern");
    let sanitized = re.replace_all(input, "").to_string();

    let final_sanitized = sanitized.replace("../", "").replace("/../", "");

    println!("Sanitized path: {}", final_sanitized);

    final_sanitized
}

// Validation function using string methods and regular expressions
// Validation function using string methods and regular expressions
pub fn validate_input(request: &Requests) -> Result<(), ValidationParseError> {
    let path = request.path();
    let query_string = request.query_string();

    // Simple path validation using string methods
    if path.contains("..") || path.contains("/./") || path.contains("//") {
        return Err(ValidationParseError::VulnerablePath);
    }

    // Regex-based query string validation (Corrected regex pattern)
    if let Some(query_string) = query_string {
        let vulnerable_chars = Regex::new(r#"[\";]"#).expect("Invalid regex pattern");

        if vulnerable_chars.is_match(query_string) {
            return Err(ValidationParseError::VulnerableQueryString);
        }
    }

    Ok(())
}
