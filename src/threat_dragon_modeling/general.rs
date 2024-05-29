use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Config, ConfigError, ConfigYaml},
    input::{input_diagram::InputDiagram, threat::Threat},
    process::process::{MappingFromInputDiagram, MappingFromVecInputDiagram},
};

use super::diagram::Diagram;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThreatModeling {
    pub version: String,
    pub summary: Summary,
    pub detail: Detail,
}

impl ThreatModeling {
    pub fn new(input_diagram: &Vec<InputDiagram>, config: &Config, threats: &Vec<Threat>) -> Self {
        Self {
            version: config.threat_dragon_version.clone(),
            summary: Summary::new(config),
            detail: Detail::from_input_diagram(&input_diagram, &config, threats),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Summary {
    pub title: String,
    pub owner: String,
    pub description: String,
    pub id: u32,
}

impl Summary {
    fn new(config: &Config) -> Self {
        Self {
            title: config.title.clone(),
            owner: config.owner.clone(),
            description: config.description.clone(),
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
pub struct Detail {
    pub contributors: Vec<String>,
    pub diagrams: Vec<Diagram>,
    pub diagram_top: u32,
    pub reviewer: String,
    pub threat_top: u32,
}

impl MappingFromVecInputDiagram for Detail {
    fn from_input_diagram(
        input_diagram: &Vec<InputDiagram>,
        config: &Config,
        threats: &Vec<Threat>,
    ) -> Self {
        let mut json_diagram: BTreeMap<String, Diagram> = input_diagram
            .iter()
            .map(|input_diagram| {
                (
                    input_diagram.title.clone(),
                    Diagram::from_input_diagram(&input_diagram, &config, None, threats),
                )
            })
            .collect();

        config.diagrams.iter().for_each(|config_diagram| {
            let diagram_parent = input_diagram
                .iter()
                .filter(|input_diagram| input_diagram.title == config_diagram.parent)
                .last();
            // Add childs diagrams
            if let Some(parent) = diagram_parent {
                json_diagram.insert(
                    format!("{}_{}", parent.title.clone(), config_diagram.name.clone()),
                    Diagram::from_input_diagram(
                        &parent,
                        &config,
                        Some(config_diagram.name.clone()),
                        threats,
                    ),
                );
            }
        });

        Self {
            contributors: Vec::new(),
            diagrams: json_diagram.values().cloned().collect(),
            diagram_top: 0,
            reviewer: "".to_string(),
            threat_top: 0,
        }
    }
}
