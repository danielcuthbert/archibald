use archibald_handler::ArchibaldHandler;
/// Archibald is a web server written in Rust
///
/// This is the main file for the server
/// Author: @danielcuthbert
///
/// The corresponding threat model for this can be found in the same repo
use http::methods::Allowedmethods;
use log::{error, info, warn, LevelFilter};
use server::archibaldserver::Server;
mod archibald_handler;
mod http;
mod server;

use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, process::exit};

// Our top level struct to hold the config file
#[derive(Deserialize)]
struct Data {
    config: ArchibaldConfig,
}

#[derive(Serialize, Deserialize)]
struct ArchibaldConfig {
    static_path: String,
    default_path: String,
    ip: String,
    port: u16,
}

// Start of the main function

fn main() {
    // Variable that holds the filename as a `&str`.
    let archibald_toml = "config/archibald.toml";

    // Read the contents of the file using a `match` block
    // to return the `data: Ok(c)` as a `String`
    // or handle any `errors: Err(_)`.
    let contents = match fs::read_to_string(archibald_toml) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            println!("Could not read file `{}`", archibald_toml);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // Use a `match` block to return the
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.
    let data: Data = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(e) => {
            println!("Unable to load data from `{}`", archibald_toml);
            println!("Please check the file is valid TOML, {}", e);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // Print out the values to `stdout`.
    println!(" Loading Archibald on: {}", data.config.ip);
    println!(" Listening port is: {}", data.config.port);

    let archibald = Server::new(format!("{}:{}", data.config.ip, data.config.port));
    let _get = Allowedmethods::GET;
    let _post = Allowedmethods::POST;
    archibald.run(ArchibaldHandler::new(data.config.default_path));
}

//We need to create a server and read the port number from the config file
