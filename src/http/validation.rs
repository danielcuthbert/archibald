// src/http/validation.rs

use crate::http::errors::ParseError;
use log::debug;
use percent_encoding::percent_decode_str;
use std::path::{Component, Path, PathBuf};

pub fn validate_input(input: &str) -> Result<PathBuf, ParseError> {
    debug!("Validating input path: {}", input);

    // URL-decode the input path
    let decoded_input = percent_decode_str(input)
        .decode_utf8()
        .map_err(|_| ParseError::InvalidEncoding)?;
    let decoded_input = decoded_input.as_ref();

    debug!("Decoded input path: {}", decoded_input);

    // Start with an empty path
    let mut path = PathBuf::new();

    // Sanitize the input path by filtering out dangerous components
    for component in Path::new(decoded_input).components() {
        match component {
            Component::Normal(name) => {
                // Append valid path components
                path.push(name);
            }
            Component::RootDir => {
                // Skip the root directory component to create a relative path
                // This allows paths starting with '/' without including it in the PathBuf
                continue;
            }
            Component::CurDir => {
                // Ignore the current directory component '.'
                continue;
            }
            Component::ParentDir => {
                // Reject attempts to navigate to parent directories
                debug!("Invalid path component (ParentDir): {:?}", component);
                return Err(ParseError::InvalidPath);
            }
            _ => {
                // Reject any other components (e.g., prefix, verbatim)
                debug!("Invalid path component: {:?}", component);
                return Err(ParseError::InvalidPath);
            }
        }
    }

    // If the path is empty after processing, default to "index.html"
    if path.as_os_str().is_empty() {
        path.push("index.html");
    }

    debug!("Sanitized path: {:?}", path);

    Ok(path)
}
