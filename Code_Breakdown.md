The 'src' directory contains several Rust source files:

    main.rs
    server.rs
    archibald_handler.rs
    settings.rs

And a directory named 'http'.

### http 

The 'http' directory contains several Rust source files as well:

    errors.rs
    validation.rs
    arch_requests.rs
    methods.rs
    mod.rs
    arch_response.rs
    query_string.rs
    statuscodes.rs
    
### main.rs    

The main.rs file contains the main function that starts the web server. It does the following:

*  Deserializes a TOML configuration file (archibald.toml) into a struct Data that has a field config of type ArchibaldConfig. This configuration contains the static path, default path, IP address, and port number.
*  Prints the IP address and port number, which are used to start the server.
*  Creates a new server with the IP and port from the configuration.
*  Creates GET and POST HTTP method objects. However, these objects aren't currently used.
*  Runs the server with a new ArchibaldHandler that is initialized with the default path from the configuration.

### server.rs

The server.rs file contains the definition of the Server struct and the ServerHandler trait. 

It sets up the basic structure for a TCP server that listens for incoming connections, reads requests, and sends responses. The handle_request_internal method of the ServerHandler trait returns a Result<Response, ParseError>, and this response is sent back to the client using the response.send(&mut stream) method.

Where and how the Response struct is being used in the server logic:

    Handling Requests: When a request is received and parsed successfully, handle_request_internal is called. This method presumably constructs a Response object based on the request and application logic.

    Sending Responses: The constructed Response is then sent back to the client using the send method of the Response struct. This is where the HTTP response, including the status line and headers, is formatted and written to the client's stream.

    Handling Bad Requests: If the request parsing fails, The error is printed out.

Here's a brief overview of their functionality:

The ServerHandler trait defines three methods that a server handler should implement: handle_request, handle_bad_request, and handle_request_internal.

The Server struct has a single field, address, which is the address on which the server is running.

The Server struct also has two methods:
*         new creates a new Server instance.
*         run starts the server, accepting TCP connections and handling requests.   

The archibald_handler.rs file contains the ArchibaldHandler struct and its implementation of the ServerHandler trait.

The ArchibaldHandler struct has a single field, static_path, which is the path to the directory containing the static files.

The struct has two methods:

    new, which creates a new ArchibaldHandler instance with the given static path.
    read_file, which reads a file from the static directory. It takes a file path as an argument and returns the contents of the file as a String.

The ServerHandler trait implementation includes:

    handle_request, which handles incoming HTTP requests. It checks the request path and returns the appropriate response.
    handle_bad_request and handle_request_internal, which are currently unimplemented and are left as stubs.       

### settings.rs    

The settings.rs file seems to contain the configuration settings for the web server. It includes:

    Constants specifying the location of the configuration file.
    Log, Server, and Port structs to hold various configuration values.
    ENV enum to specify the environment (Development, Production, or Testing).
    Implementation of fmt::Display for the ENV enum.
    A Settings struct with a new method to create a new settings instance.

The new method:

    Reads an environment variable to determine the run environment.
    Merges settings from a default configuration file and an environment-specific configuration file.
    Merges settings from environment variables with a specific prefix and separator.
    Attempts to convert the configuration into the Settings struct.    
    
### errors.rs    

The errors.rs file contains the definition of the ParseError enum, which represents potential errors that can occur when parsing an HTTP request. The different kinds of errors are:

    NotFound: The requested resource was not found.
    InvalidRequest: The request is invalid.
    InvalidMethod: The HTTP method is invalid.
    InvalidHeader: The HTTP header is invalid.
    InvalidBody: The HTTP body is invalid.
    InvalidEncoding: The encoding is invalid.
    InvalidProtocol: The HTTP protocol is invalid.

It also includes implementations of the Display and Debug traits for ParseError, which format the error for display.

Additionally, there are From trait implementations for MethodError and Utf8Error. These implementations allow a MethodError or Utf8Error to be automatically converted into a ParseError.

The ParseError enum also implements the Error trait, which means it can be used with the ? operator in a function that returns a Result.

### valiation.rs

The validation.rs file seems to contain functions for input validation and sanitization.

There are two main parts:

    A module validation that includes:
        An enum ValidationParseError to represent different types of validation errors.
        A function validate_input that checks if a request's method, path, and query string are valid.

    A function sanitize_input that removes invalid characters from a string.

There are a few issues in this file:

    The validate_input function attempts to destructure query_string from request.query_string(). However, query_string is not defined in this context. The commented-out line before this section suggests there might have been an attempt to do this but it's commented out.
    Also, the query_string is expected to have keys() method, implying it's a type of map or similar, which needs to be checked in the context of Requests.
    The ValidationParseError enum should implement the Error trait and possibly the Display and Debug traits. Implementing these traits would allow the ValidationParseError to be used more effectively with error handling idioms in Rust.
    The sanitize_input function seems to be outside of the validation module, which might be a mistake depending on the expected organization of the module.

### arch_requests.rs
    
The arch_requests.rs file defines a Requests struct and some related functions.

The Requests struct represents an HTTP request and contains the following fields:

    path: The path of the request.
    reqpath: A second path field, although it's not clear how this differs from the path field.
    method: The HTTP method of the request.

The Requests struct also has several methods, including:

    path: Returns the path of the request.
    method: Returns the HTTP method of the request.
    validate_input: Validates the path of the request.

There's also an implementation of TryFrom<&[u8]> for Requests. This allows a Requests instance to be created from a byte slice, which presumably represents an incoming HTTP request. The implementation attempts to parse the HTTP method, protocol, and query string from the request.

There's an embedded validation module that includes two functions:

    sanitize_input: Sanitizes a string by removing non-alphanumeric characters.
    validate_input: Validates a path by sanitizing it and then validating the sanitized string.

Here are some potential issues:

    There are two fields path and reqpath in Requests struct which seem to serve the same purpose. It's not clear why both are needed.
    The Requests struct has a method path which seems to recursively call itself, leading to an infinite recursion.
    The Requests struct seems to be missing a query_string field, but it's used in the try_from implementation.
    The path field in Requests struct is set as todo!() in the try_from implementation. This will cause the program to panic when a Requests object is created from a byte slice.
    The validate_input method in the Requests struct calls validation::sanitize_input, which returns a Result. However, it doesn't handle the Result, which will cause a compile error.

### methods.rs
The methods.rs file contains an enumeration Allowedmethods for the different types of HTTP methods that are supported. It also includes an implementation of FromStr for Allowedmethods, which allows an HTTP method to be parsed from a string.

The Allowedmethods enumeration has the following methods:

    is_valid: Checks if the method is valid (currently always returns true).
    as_str: Returns a string representation of the method (currently always returns "GET").

The MethodError struct represents an error that occurs when an invalid HTTP method is parsed. It implements the Display and Error traits.

There are a few potential issues with this file:

    The is_valid method of Allowedmethods always returns true, which might not be the desired behavior.
    The as_str method of Allowedmethods always returns "GET", regardless of the actual method.
    There's a use statement at the beginning of the file that attempts to import Allowedmethods under a different name OtherAllowedmethods, but it's not used anywhere in the file.
    
### mod.rs    
The mod.rs file in the http directory is used to declare the public interface of the http module. It includes:

    pub mod declarations for the submodules errors, methods, query_string, arch_requests, arch_response, statuscodes, and validation.
    pub use declarations to re-export certain items from these submodules, including ParseError, QueryString, ValueofQueryString, Response, and StatusCode
    
### arch_response.rs

The arch_response.rs file defines a Response struct that represents an HTTP response.

The Response struct has the following fields:

    body: An optional body of the response. It's declared as an Option<String> which allows the body to be None.
    status_code: The HTTP status code of the response.
    statusmessage: A string representation of the HTTP status code.

The Response struct also has two methods:

    new: Creates a new Response instance with the given status code and optional body.
    send: Sends the response to a given stream.  
    
###query_string.rs

The query_string.rs file contains definitions for parsing and representing a query string from an HTTP request.

The QueryString struct represents a parsed query string. It contains a HashMap that maps keys to values from the query string.

There's also an enum Value which can be either Single or Multiple, presumably to accommodate query parameters that have multiple values.

The impl From<&str> for QueryString implementation allows a QueryString instance to be created from a string slice. The from method splits the string on the & character to separate the key-value pairs, and then further splits each pair on the = character to separate the keys from the values. The keys and values are then inserted into the HashMap.

Here are some potential issues I need to work on:

*     There's commented-out code that seems to be intended for handling keys with multiple values. This isn't currently implemented.
*     The QueryString struct's data field uses String as the value type. This means it can't currently handle multiple values for a key. If you want to support multiple values for a key, you might need to change this to HashMap<String, Value>.
*     The Value enum isn't currently used anywhere. This might be related to the commented-out code for handling multiple values for a key.          

### statuscodes.rs

The statuscodes.rs file defines an enumeration StatusCode that represents HTTP status codes. Each variant in the enumeration is associated with a specific numeric code.

The StatusCode enumeration includes the following implementations:

*     Debug and Clone traits derived for StatusCode.
*     Into<u16> trait implemented for StatusCode. This allows a StatusCode to be converted into a u16.
*     http_status_reason_phrase method which maps each StatusCode to its respective HTTP reason phrase.
*     Display trait implemented for StatusCode. This allows a StatusCode to be formatted as a string.