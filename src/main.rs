use archibald_handler::ArchibaldHandler;
/// Archibald is a web server written in Rust
///
/// This is the main file for the server
/// Author: @danielcuthbert
///
/// The corresponding threat model for this can be found in the same repo
use http::methods::Allowedmethods;

use log::{info, warn, LevelFilter};
use server::archibaldserver::Server;
mod archibald_handler;
mod http;
mod server;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Serialize, Deserialize)]
struct ArchibaldConfig {
    static_path: String,
    default_path: String,
    ip: String,
    port: u32,
}

// Start of the main function

fn main() {
    #[cfg(debug_assertions)]
    if let Some(proj_dirs) = ProjectDirs::from("dev", "danielcuthbert", "archibald") {
        let config_dir = proj_dirs.config_dir();
        let archibald_config = fs::read_to_string(config_dir.join("archibald.toml"));
        let config: ArchibaldConfig = match archibald_config {
            Ok(config) => toml::from_str(&config).unwrap(),
            Err(_) => {
                warn!("No config file found, using defaults");
                ArchibaldConfig {
                    static_path: format!("{}/static_content", env!("CARGO_MANIFEST_DIR")),
                    default_path: format!(
                        "{}/static_content/index.html",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    ip: "127.0.0.1".to_string(),
                    port: 8080,
                }
            }
        };

        //We need to create a server and read the port number from the config file
        let archibald = Server::new(format!("{}:{}", config.ip, config.port));

        // Tell the server what HTTP methods we want to support
        let _get = Allowedmethods::GET;
        let _post = Allowedmethods::POST;

        // Start the server, this will always run
        archibald.run(ArchibaldHandler::new(config.default_path));
    }
}
