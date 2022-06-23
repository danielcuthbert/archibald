/*
* Archibald: a loyal web server
* Main response module
* Author: @danielcuthbert
*
* This code serves as the response function.
*/

// We define a struct to hold all the data we need to send back to the client
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    pub statuscode: u16,
    pub statusmessage: String,
    pub body: Option<String>, //there might be no body so we can use Option<>
}

impl Response {
    // here we use a new public function that takes the above struct and returns a string
    pub fn new(statuscode: u16, statusmessage: String, body: Option<String>) -> Self {
        Response {
            statuscode,
            statusmessage,
            body,
        }
    }
}
