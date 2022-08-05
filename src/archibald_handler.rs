/*
* Archibald: a loyal web server
* This holds the implementation of the handler
* Author: @danielcuthbert
*
*/

// use crate::http::statuscodes;

use crate::http::methods::Allowedmethods;
use crate::http::requests::Request;
use response::Response;

// We make use of a Archibald Handler
use super::http::{methods, requests, response, statuscodes};
// use super::http::response::Response;
// use super::http::Methods;
// use super::http::StatusCode;
use super::http::statuscodes::StatusCode::{JollyGood, NotFound};
use super::server::archibaldserver::ServerHandler;

// this is the main handler module
pub struct ArchibaldHandler;

impl ServerHandler for ArchibaldHandler {
    // this handles the request
    fn handle_request(&mut self, request: &Request) -> Response {
        println!("METHOD {:?} PATH '{}'", request.method(), request.path());
        match request.method() {
            // We need to handle the requests depending on what they are. This is where we do that.
            Allowedmethods::GET => match request.path() {
                // If the path is /, we want to return a simple string
                "/" => Response::new(JollyGood, Some("I say old boy!!".to_string())),
                _ => Response::new(NotFound, Some("Not Found".to_string())),
            },
            // If the query is None, we return a 404 error
            _ => Response::new(NotFound, Some("Not Found".to_string())),
        }
    }

    fn handle_bad_request(&mut self, e: &crate::http::errors::ParseError) -> Response {
        todo!()
    }
}
