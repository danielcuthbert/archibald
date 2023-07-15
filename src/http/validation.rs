/*
* Archibald: a loyal web server
* Main validation module
* Author: @danielcuthbert
* Makes use of https://serde.rs/

This code defines the validate_input() method, which takes a Request object as an argument and returns a Result. 
The validate_input() method checks if the request method is valid, the path contains any malicious characters, 
and the query string contains any malicious characters. 

If any of these checks fail, the validate_input() method returns an error. 
Otherwise, the validate_input() method returns a Ok value.

The validate_input() method is then imported by the requests.rs file, which can then call the validate_input() method to validate incoming requests.

*/

use crate::http::{arch_requests::Requests, ParseError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::{self, Regex};

mod validation {
    
    use crate::http::{arch_requests::Requests, methods::Allowedmethods, ParseError};

    


    pub fn validate_input(request: &Requests) -> Result<(), ParseError> {
        let method = request.method().as_str();
        let path = request.path();
        let query_string = request.query_string();

        // Check if the method is valid
        if !Allowedmethods::is_valid(method) {
            return Err(ParseError::InvalidMethod);
        }

        // Check if the path contains any malicious characters
        if path.contains("'") || path.contains("\"") || path.contains(";") {
            return Err(ParseError::MaliciousPath);
        }

        // Check if the query string contains any malicious characters
        if let Some(query_string) = query_string {
            for key in query_string.keys() {
                if key.contains("'") || key.contains("\"") || key.contains(";") {
                    return Err(ParseError::MaliciousQueryString);
                }
            }
        }

        Ok(())
    }

    pub fn sanitize_input(input: &str) -> String {
        let mut sanitized_input = String::new();
    
        for character in input.chars() {
            if !Regex::new(r"[^\w\s]").unwrap().is_match(&character.to_string()) {
                sanitized_input.push(character);
            }
        }
    
        return sanitized_input;
    }

}
