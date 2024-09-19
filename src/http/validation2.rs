// This is a alpha version of the main validation.rs file. 
// It will hopefully stop basic encoding attacks, NULL bytes and double encoding, but and there's always one
// It probably wont stop:
// - sym link attacks
// - Overlong UTF-8 Encoding
// - Non-Standard Encodings because i use percent_encoding
// - Complex Nested Encodings and Unicode Normalization and yes im going deep af
// - Probably dos stuff too, like %25%25%25%25%25%25%25%25%25%25%25%25 where i just cause Archibald to fall over
// - Oh chained ones too, yeah they are hard

use std::path::Path;
use std::ffi::OsStr;
use percent_encoding::percent_decode_str; // https://docs.rs/crate/percent-encoding/latest
use std::fs;

use std::path::Path;
use std::ffi::OsStr;
use percent_encoding::percent_decode_str;
use log::{info, warn, error};  // Import logging macros

// Function to recursively decode URL-encoded characters
fn recursive_decode(url: &str) -> String {
    let mut decoded_url = url.to_string();
    loop {
        let decoded_once = percent_decode_str(&decoded_url)
            .decode_utf8_lossy()
            .to_string();
        if decoded_once == decoded_url {
            break;
        }
        decoded_url = decoded_once;
    }
    decoded_url
}

// Function to check if a string contains a null byte, there are ways to bypass this but this should suffice
fn contains_null_byte(input: &str) -> bool {
    input.contains('\0') || input.contains("%00")
}

// this is the monster, it should catch a lot but not all (see above)
fn validate_path(path: &str) -> bool {
    // Step 1: Recursively decode the URL to handle double and mixed encodings
    let decoded_url = recursive_decode(path);
    // Step 2: Log the incoming path for auditing purposes
    info!("Received request for path: {}", decoded_url);
    // Step 3: Reject any URL that contains a null byte, see above
    if contains_null_byte(&decoded_url) {
        error!("Rejected: Null byte detected in path: {}", decoded_url); // we will log all attempts 
        return false;
    }
    // Step 4: Construct a Path object from the decoded URL
    let path_obj = Path::new(&decoded_url);
    // Step 5: Canonicalize the path to resolve any ".." or symbolic links
    let canonical_path = match path_obj.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            error!("Rejected: Invalid or unsafe path detected: {}", decoded_url);
            return false;
        }
    };
    // Step 6: Define the allowed directory where files can be served from
    let allowed_directory = Path::new("/static_content").canonicalize().unwrap();
    // Step 7: Check if the canonical path is within the allowed directory
    if !canonical_path.starts_with(&allowed_directory) {
        error!("Rejected: Path outside allowed directory: {}", canonical_path.display());
        return false;
    }
    // Step 8: Check for dangerous file extensions if needed, yes this is heavily abused so YMMV
    let valid_extensions = vec!["html", "css", "js", "png", "jpg"];
    if let Some(extension) = canonical_path.extension().and_then(OsStr::to_str) {
        if !valid_extensions.contains(&extension) {
            error!(
                "Rejected: Invalid file extension '{}', path: {}",
                extension, canonical_path.display()
            );
            return false;
        }
    } else {
        error!("Rejected: No valid file extension found for path: {}", canonical_path.display());
        return false;
    }
    // If we get here, happy damn days
    // All checks passed, log the successful access
    info!("Successfully validated and allowed path: {}", canonical_path.display());
    true
}

// Test function to validate several paths with logging enabled
#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    #[test]
    fn test_path_validation() {
        // Initialize logging for testing
        let _ = env_logger::builder().is_test(true).try_init();

        let test_cases = vec![
            ("/index.html", true),  // Safe file
            ("/..%2fsecret.txt", false),  // Mixed encoding, directory traversal
            ("/index.html%00.png", false),  // Null byte injection
            ("/../../etc/passwd", false),  // Plain directory traversal
            ("/allowed/file.html", true),  // Valid file in allowed directory
            ("/allowed/secret.txt", false),  // Invalid file extension
        ];

        for (path, expected_result) in test_cases {
            let result = validate_path(path);
            assert_eq!(result, expected_result, "Path: {}", path);
        }
    }
}
