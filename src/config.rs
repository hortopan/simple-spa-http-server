use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_cache_control")]
    pub cache_control: String,
    #[serde(default = "default_serve")]
    pub serve: String,
}

fn default_cache_control() -> String {
    "public, max-age=86400".to_string()
}

fn default_bind() -> String {
    "0.0.0.0:8080".to_string()
}

fn default_serve() -> String {
    "./www".to_string()
}

impl Config {
    pub fn from_env() -> Self {
        match envy::from_env::<Config>() {
            Ok(config) => config,
            Err(error) => panic!("Missing environment variable: {:#?}", error),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env();
}
