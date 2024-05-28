use std::collections::BTreeMap;

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
            summary: Summary::new(config),
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
struct Detail {
    contributors: Vec<String>,
    diagrams: Vec<Diagram>,
    diagram_top: u32,
    reviewer: String,
    threat_top: u32,
}

impl MappingFromVecInputDiagram for Detail {
    fn from_input_diagram(input_diagram: &Vec<InputDiagram>, config: &Config) -> Self {
        let mut json_diagram: BTreeMap<String, Diagram> = input_diagram
            .iter()
            .map(|input_diagram| {
                (
                    input_diagram.title.clone(),
                    Diagram::from_input_diagram(&input_diagram, &config, None),
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
