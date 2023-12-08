use crate::http::arch_requests::Requests;
use crate::http::methods::Allowedmethods;
use regex::Regex;

use std::fmt::{Display, Formatter};

// Assuming you have a module `arch_requests` with a struct `Requests`
// and a module `methods` with an enum `Allowedmethods`

mod validation {
    use super::*; // To bring types like Allowedmethods into scope
    use std::error::Error;

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
                ValidationParseError::MaliciousQueryString => write!(f, "Malicious query string detected"),
                ValidationParseError::InvalidRegex => write!(f, "Invalid regex pattern"),
            }
        }
    }

    impl Error for ValidationParseError {}

    pub fn validate_input(request: &Requests) -> Result<(), ValidationParseError> {
        let method = request.method();
        let path = request.path();
        let query_string = request.query_string();
    
        // Check if the method is valid
        if !Allowedmethods::is_valid(method) {
            return Err(ValidationParseError::InvalidMethod);
        }
    
        // Check if the path contains any malicious characters
        if path.contains('\'') || path.contains('\"') || path.contains(';') {
            return Err(ValidationParseError::MaliciousPath);
        }
    
        // Check if the query string contains any malicious characters
        if let Some(query_string) = query_string {
            // Split the query string into key-value pairs
            for pair in query_string.split('&') {
                // Further split each pair into key and value
                let parts: Vec<&str> = pair.split('=').collect();
                if let Some(key) = parts.get(0) {
                    // Check if the key contains any malicious characters
                    if key.contains('\'') || key.contains('\"') || key.contains(';') {
                        return Err(ValidationParseError::MaliciousQueryString);
                    }
                }
            }
        }
    
        Ok(())
    }
    
pub fn sanitize_input(input: &str) -> String {
    // Compile regex pattern outside of the loop
    let re = Regex::new(r"[^\w\s]").expect("Invalid regex pattern");
    re.replace_all(input, "").to_string()
}

}
