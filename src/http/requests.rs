/*
* Archibald: a loyal web server
* Main requests module
* Author: @danielcuthbert
*
*/

use crate::http::methods::Allowedmethods;
pub struct Request {
    // We need to store the request body
    method: Allowedmethods,
    query: Option<String>, // This is a string that can be None
    path: String,
    body: String,
    statuscode: u16,
    statusmessage: String,
}
