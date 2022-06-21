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

// Derive function so we can use dbg!()

// we should use a lifetime for this as both key and value will live in the same buffer and we want to control this from a memory safety standpoint
// in requests.rs, we named it 'a so will continue here.
#[derive(Debug)]
pub struct QueryString {
    // we will be using a hashmap (key:value pairs) for this
    data: HashMap<String, String>,
}

// Setting the stage for single and multiple query string values
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Single(String),
    Multiple(Vec<String>),
}

// the query string stores references
//

// We want to convert from a string slice to a query string
impl From<&str> for QueryString {
    //lifetimes are required for this to work
    fn from(query: &str) -> Self {
        // here is where we use an empty hashmap again to dump stuff into

        let mut qs = QueryString {
            data: HashMap::new(),
        };

        // we need to split the query string on the & and do stuff with it
        // the method s.split works perfectly here
        // we use a for loop to reiterate over the query string
        // <K>=<V> & <K>=<V>
        query.split('&').for_each(|query_string| {
            let mut pair = query_string.split('=');
            let key = pair.next().unwrap();
            let value = pair.next().unwrap();
            //s.find('=').unwrap(); // this looks for any use of an = sign in the query string
            qs.data.insert(key.to_string(), value.to_string());
        });
        return qs;
    }
}

// We need to somehow query the data to see if a key/value has already been stored, if not, add it
// it does this using the or_insert and adds the key/value we specify to the hashmap
// I feel this is very complicated and I'm sure it could be done better
/*
data.entry(key)
.and_modify(|existing|match existing {
    Value::Single(existing) => {
        data.insert(key, value);
    }
    Value::Multiple(existing) => {
        data.insert(key, value);
    }
}
.or_insert(Value::Single(value)),
);
*/
//},
//}
