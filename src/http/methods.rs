/*
* Archibald: a loyal web server
* Main methods module
* Author: @danielcuthbert
*
*/

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Allowedmethods {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

impl FromStr for Allowedmethods {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Allowedmethods::GET),
            "POST" => Ok(Allowedmethods::POST),
            "PUT" => Ok(Allowedmethods::PUT),
            "DELETE" => Ok(Allowedmethods::DELETE),
            "HEAD" => Ok(Allowedmethods::HEAD),
            "OPTIONS" => Ok(Allowedmethods::OPTIONS),
            "PATCH" => Ok(Allowedmethods::PATCH),
            "TRACE" => Ok(Allowedmethods::TRACE),
            "CONNECT" => Ok(Allowedmethods::CONNECT),
            _ => Err(MethodError),
        }
    }
}

#[derive(Debug)]
pub struct MethodError;

impl std::fmt::Display for MethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid HTTP method")
    }
}

impl std::error::Error for MethodError {}
