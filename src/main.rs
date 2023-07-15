use archibald_handler::ArchibaldHandler;

use http::methods::Allowedmethods;
use log::LevelFilter;

use server::archibaldserver::Server;

mod archibald_handler;
mod http;
mod server;

use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, process::exit};

#[derive(Deserialize)]
struct ArchibaldConfig {
    static_path: String,
    default_path: String,
    ip: String,
    port: u16,
}

#[derive(Deserialize)]
struct Data {
    config: ArchibaldConfig,
}

fn main() {
    let archibald_toml = "config/archibald.toml";

    let contents = match fs::read_to_string(archibald_toml) {
        Ok(c) => c,
        Err(e) => {
            println!("Could not read file `{}`: {}", archibald_toml, e);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            println!("Unable to load data from `{}`: {}", archibald_toml, e);
            exit(1);
        }
    };

    println!("Loading Archibald on: {}", data.config.ip);
    println!("Listening port is: {}", data.config.port);

    let archibald = Server::new(format!("{}:{}", data.config.ip, data.config.port));
    let _get = Allowedmethods::GET;
    let _post = Allowedmethods::POST;
    archibald.run(ArchibaldHandler::new(data.config.default_path));
}