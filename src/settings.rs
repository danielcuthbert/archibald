/*
* Archibald: a loyal web server
* Main settings file
* Author: @danielcuthbert
*
*/

use config::{Config, ConfigError, Environment, File};

// Here we set where the config file is located

const CONFIG_FILE: &str = "./config/archibald.toml";
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

// We should define if the server is in production or development mode (or testing too)

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

// Now we have to actually implement the settings bit

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Check whether a RUN_ENV environment variable was set (if not, default to Development)
        let env = env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        // Create a default config::Config and manually set the ENV variable.
        let mut s = Config::new();
        s.set("environment", env.clone())?;
        // We used the merge function, first with the default config file and
        // then with the file selected based on the given environment (e.g.: ./config/archibald.toml).
        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        // The Environment::with_prefix option ensures that any environment variable
        // that is prefixed with EA and matches one of our config paths is used
        s.merge(Environment::with_prefix("ea").separator("__"))?;

        s.try_into()
    }
}
