/*
* Archibald: a loyal web server
* This holds the implementation of the handler
* Author: @danielcuthbert
*
*/

// use crate::http::statuscodes;

use response::Response;

use crate::http::requests::Request;

// We make use of a Archibald Handler
use super::http::{methods, requests, response, statuscodes};
// use super::http::response::Response;
// use super::http::Methods;
// use super::http::StatusCode;
use super::http::StatusCode::NOT_FOUND;
use super::server::archibaldserver::ServerHandler;

// this is the main handler module
pub struct ArchibaldHandler;

impl ServerHandler for ArchibaldHandler {
    // this handles the request
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            // We need to handle GET requests
            Method::GET => match request.path() {
                // If the query is None, we return a 404
                None => Response::new(NOT_FOUND, "Not Found"),
                // we can also do the stock /
                "/" => Response::new(statuscodes::OK, "Hello Master, how can I help you?"),
                // If the query is Some, we return the query
                Some(query) => Response::new(statuscodes::OK, Some(query.to_string())),
            },
        }
    }

    fn handle_bad_request(&mut self, e: &crate::http::errors::ParseError) -> Response {
        todo!()
    }
}
