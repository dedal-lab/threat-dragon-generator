use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub threat_dragon_version: String,
    pub title: String,
    pub owner: String,
    pub description: String,
    pub trust_boundaries: Vec<TrustBoundary>,
    pub diagrams: Vec<Diagrams>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrustBoundary {
    pub name: String,
    pub description: String,
    pub limit_of_access: String,
    pub level_of_authorization: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Diagrams {
    pub name: String,
    pub parent: String,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    UnknownError(String),
}

pub trait ConfigYaml {
    fn from_config(&mut self, config: &Config) -> Result<(), ConfigError>;
}
