/*
* Archibald: a loyal web server
* Main methods module
* Author: @danielcuthbert
*
*/


use std::str::FromStr; // This trait parse a value from a string

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

// This function receives a string that contains the method from the request and we need to convert it


impl FromStr for Allowedmethods {
    type Err = MethodError;

    // This function receives a string that contains a method from the request
    // We need to convert this
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
            //_ => Err(format!("So sorry old chap, that method is invalid: {}", s)),
            _ => Err(MethodError),
        }
    }
}

// We need a custom error type for the request parser

pub struct MethodError;



