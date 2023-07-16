use crate::http::methods::Allowedmethods as OtherAllowedmethods;
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

impl Allowedmethods {
    pub fn is_valid(&self) -> bool {
        match self {
            Allowedmethods::GET => true,
            Allowedmethods::POST => true,
            Allowedmethods::PUT => true,
            Allowedmethods::DELETE => true,
            Allowedmethods::HEAD => true,
            Allowedmethods::OPTIONS => true,
            Allowedmethods::PATCH => true,
            Allowedmethods::TRACE => true,
            Allowedmethods::CONNECT => true,
        }
    }

    pub fn as_str(&self) -> &str {
        "GET"
    }
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
