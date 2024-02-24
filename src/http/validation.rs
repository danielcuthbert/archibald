use crate::http::arch_requests::Requests;
use pest::Parser; // The Parser trait
use pest::error::Error as PestError; // The PestError type
use pest_derive::Parser; // The Parser derive macro
use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};


use path_grammar::Rule;
use query_string_grammar::Rule;

#[derive(Parser)]
#[grammar = "./path_grammar.pest"]
struct PathParser;

#[derive(Parser)]
#[grammar = "./query_string_grammar.pest"]
struct QueryStringParser;

impl pest::Parser<Rule> for PathParser {}
impl pest::Parser<Rule> for QueryStringParser {}

#[derive(Debug)]
pub enum ValidationParseError<'a> {
    InvalidMethod,
    MalformedPath(PestError<Rule<'a>>),
    MalformedQueryString(PestError<Rule<'a>>),
    VulnerablePath,
    VulnerableQueryString,
}

impl<'a> Display for ValidationParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            ValidationParseError::InvalidMethod => write!(f, "Invalid method"),
            ValidationParseError::VulnerablePath => write!(f, "Malicious path detected"),
            ValidationParseError::VulnerableQueryString => write!(f, "Malicious query string detected"),
            ValidationParseError::MalformedPath(_) => write!(f, "Malformed path: {}", self),
            ValidationParseError::MalformedQueryString(_) => write!(f, "Malformed query string: {}", self),
            // Add handling for other cases as needed
        }
    }
}

impl<'a> Error for ValidationParseError<'a> {}

pub fn sanitize_input(input: &str) -> String {
    println!("Original path: {}", input); // Log the original path

    let re = Regex::new(r"[^\w\s./-]").expect("Invalid regex pattern");
    let sanitized = re.replace_all(input, "").to_string();

    // Replace any '..' sequences to prevent directory traversal attacks
    let final_sanitized = sanitized.replace("../", "").replace("/../", "");

    println!("Sanitized path: {}", final_sanitized); // Log the sanitized path

    final_sanitized
}

pub fn validate_input<'a>(request: &'a Requests) -> Result<(), ValidationParseError<'a>> {

    let path = request.path();
    let query_string = request.query_string();

    // Validate the path using the PathParser
    match PathParser::parse(Rule::path, path) {
        Ok(_) => {}
        Err(err) => return Err(ValidationParseError::MalformedPath(err)),
    }

    // Only proceed if there is a query string
    if let Some(query_string) = query_string {
        let query_parser = QueryStringParser::parse(Rule::query_string, query_string)
            .map_err(ValidationParseError::MalformedQueryString)?;

        // Iterate through the parsed key-value pairs
        for pair in query_parser {
            let key = pair.as_str(); // Adjust based on your grammar
            let value = pair.as_str(); // Adjust based on your grammar

            // Basic checks for vulnerabilities in keys and values
            if key.contains('\'') || key.contains('\"') || key.contains(';') || value.contains('\'') || value.contains('\"') || value.contains(';') {
                return Err(ValidationParseError::VulnerableQueryString);
            }
        }
    }

    Ok(())
}
