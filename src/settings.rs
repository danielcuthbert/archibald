use ::config::{builder::DefaultState, ConfigBuilder, ConfigError, File};
use serde::Deserialize;
use std::fmt;

const CONFIG_FILE_PATH: &str = "./orders/archibald.toml";

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
pub struct WebConfig {
    pub static_root: String,
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
    pub environment: ENV,
    pub web: WebConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = ConfigBuilder::<DefaultState>::default()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .build()?;

        builder.try_deserialize::<Settings>()
    }
}
