use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub threat_dragon_version: String,
}

#[derive(Debug)]
pub enum ConfigError {
    UnknownError(String),
}

pub trait ConfigYaml {
    fn from_config(&mut self, config: &Config) -> Result<(), ConfigError>;
}
