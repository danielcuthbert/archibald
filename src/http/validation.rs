use crate::http::{arch_requests::Requests, methods::Allowedmethods};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str;
use regex::Regex;

mod validation {
    
    use crate::http::{arch_requests::Requests, methods::Allowedmethods};

    enum ValidationParseError {
        InvalidMethod,
        MaliciousPath,
        MaliciousQueryString,
    }

    pub fn validate_input(request: &Requests) -> Result<(), ValidationParseError> {
        let method = request.method().as_str();
        let path = request.path();
        let query_string = *request.query_string();

        // Check if the method is valid
        if !Allowedmethods::is_valid(method) {
            return Err(ValidationParseError::InvalidMethod);
        }

        // Check if the path contains any malicious characters
        if path.contains("'") || path.contains("\"") || path.contains(";") {
            return Err(ValidationParseError::MaliciousPath);
        }

        // Check if the query string contains any malicious characters
        if let Some(query_string) = query_string {
            for key in query_string.keys() {
                if key.contains("'") || key.contains("\"") || key.contains(";") {
                    return Err(ValidationParseError::MaliciousQueryString);
                }
            }
        }

        Ok(())
    }

    
    
    
    

}

pub fn sanitize_input(input: &str) -> String {
    use regex::Regex;
    let mut sanitized_input = String::new();

    for character in input.chars() {
        // I want to use Regex here to check if the character is a valid character
        if !Regex::new(r"[^\w\s]").unwrap().is_match(&character.to_string()) {
            sanitized_input.push(character);
        }
        
        
    }
    // return the sanitized input to the caller
    return sanitized_input;
}
