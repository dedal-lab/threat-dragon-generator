use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub threat_dragon_version: String,
    pub title: String,
    pub owner: String,
    pub description: String,
    pub diagrams: Vec<Diagrams>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Diagrams {
    pub name: String,
    pub parent: String,
    pub nodes: Vec<String>
}

#[derive(Debug)]
pub enum ConfigError {
    UnknownError(String),
}

pub trait ConfigYaml {
    fn from_config(&mut self, config: &Config) -> Result<(), ConfigError>;
}
