/*
* Archibald: a loyal web server
* Main query string module
* Author: @danielcuthbert
* This holds all the structs and functionality for the query string
*/
use std::collections::HashMap;

// Example query string for an app that uses & to separate the key and value
// /?first=Daniel&last=Cuthbert
// So the hashmap will use & and store each value in that
// I'm painfully aware this isn't ideal but for now, it's the only way I can figure it out

// we should use a lifetime for this as both key and value will live in the same buffer and we want to control this from a memory safety standpoint
// in requests.rs, we named it 'a so will continue here.
pub struct QueryString<'a> {
    // we will be using a hashmap (key:value pairs) for this
    data: HashMap<&'a str, &'a str>,
}
