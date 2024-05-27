use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Config, ConfigError, ConfigYaml},
    input::input_diagram::InputDiagram,
    process::process::{MappingFromInputDiagram, MappingFromVecInputDiagram},
};

use super::diagram::Diagram;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThreatModeling {
    version: String,
    summary: Summary,
    detail: Detail,
}

impl ThreatModeling {
    pub fn new(input_diagram: &Vec<InputDiagram>, config: &Config) -> Self {
        Self {
            version: config.threat_dragon_version.clone(),
            summary: Summary::new(),
            detail: Detail::from_input_diagram(&input_diagram, &config),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Summary {
    title: String,
    owner: String,
    description: String,
    id: u32,
}

impl Summary {
    fn new() -> Self {
        Self {
            title: "test".to_string(),
            owner: "".to_string(),
            description: "".to_string(),
            id: 0,
        }
    }
}

impl ConfigYaml for ThreatModeling {
    fn from_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        self.version = config.threat_dragon_version.clone();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Detail {
    contributors: Vec<String>,
    diagrams: Vec<Diagram>,
    diagram_top: u32,
    reviewer: String,
    threat_top: u32,
}

impl MappingFromVecInputDiagram for Detail {
    fn from_input_diagram(input_diagram: &Vec<InputDiagram>, config: &Config) -> Self {
        Self {
            contributors: Vec::new(),
            diagrams: input_diagram
                .iter()
                .map(|input_diagram| Diagram::from_input_diagram(&input_diagram, &config))
                .collect(),
            diagram_top: 0,
            reviewer: "".to_string(),
            threat_top: 0,
        }
    }
}
