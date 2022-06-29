/*
* Archibald: a loyal web server
* This holds the implementation of the handler
* Author: @danielcuthbert
*
*/

use crate::http::statuscodes;

// We make use of a Archibald Handler
use super::http::requests::Request;
use super::http::response::Response;
use super::http::StatusCode;
use super::server::archibaldserver::ServerHandler;

pub struct ArchibaldHandler;

impl ServerHandler for ArchibaldHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::OK, Some("Hello World!".to_string()))
    }

    fn handle_bad_request(&mut self, e: &crate::http::errors::ParseError) -> Response {
        todo!()
    }
}
