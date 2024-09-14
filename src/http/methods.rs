use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllowedMethods {
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

impl AllowedMethods {
    pub fn is_valid(&self) -> bool {
        match self {
            AllowedMethods::GET => true,
            AllowedMethods::POST => true,
            AllowedMethods::PUT => true,
            AllowedMethods::DELETE => true,
            AllowedMethods::HEAD => true,
            AllowedMethods::OPTIONS => true,
            AllowedMethods::PATCH => true,
            AllowedMethods::TRACE => true,
            AllowedMethods::CONNECT => true,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AllowedMethods::GET => "GET",
            AllowedMethods::POST => "POST",
            AllowedMethods::PUT => "PUT",
            AllowedMethods::DELETE => "DELETE",
            AllowedMethods::HEAD => "HEAD",
            AllowedMethods::OPTIONS => "OPTIONS",
            AllowedMethods::PATCH => "PATCH",
            AllowedMethods::TRACE => "TRACE",
            AllowedMethods::CONNECT => "CONNECT",
        }
    }
}

impl FromStr for AllowedMethods {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(AllowedMethods::GET),
            "POST" => Ok(AllowedMethods::POST),
            "PUT" => Ok(AllowedMethods::PUT),
            "DELETE" => Ok(AllowedMethods::DELETE),
            "HEAD" => Ok(AllowedMethods::HEAD),
            "OPTIONS" => Ok(AllowedMethods::OPTIONS),
            "PATCH" => Ok(AllowedMethods::PATCH),
            "TRACE" => Ok(AllowedMethods::TRACE),
            "CONNECT" => Ok(AllowedMethods::CONNECT),
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
