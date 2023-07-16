use std::fmt;
use std::env;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/archibald.toml";
const CONFIG_FILE_PREFIX: &str = "./config";

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Port {
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub enum ENV {
    Development,
    Production,
    Testing,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ENV::Development => write!(f, "development"),
            ENV::Production => write!(f, "production"),
            ENV::Testing => write!(f, "testing"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log: Log,
    pub server: Server,
    pub port: Port,
    pub environment: ENV,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // existing code
    }
}
