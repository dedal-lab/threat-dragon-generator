use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::config::Config,
    input::{input_diagram::Node, threat},
    process::process::{
        MappingFromInputNode, MappingFromInputNodeAndThreats, MappingFromInputThreat,
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CellData {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_scope: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_out_of_scope: Option<String>,
    pub has_open_threats: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bidirectional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public_network: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handles_card_payment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handles_goods_or_services: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_web_application: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privilege_level: Option<String>,
    pub threats: Vec<Threat>,
}

impl MappingFromInputNodeAndThreats for CellData {
    fn from_input_diagram(
        input_node: &Node,
        config: &Config,
        threats: &Vec<threat::Threat>,
    ) -> Self {
        Self {
            type_field: format!(
                "tm.{}",
                input_node
                    .type_node
                    .to_string()
                    .chars()
                    .next()
                    .map(|c| c.to_uppercase().collect::<String>()
                        + &input_node.type_node.to_string()[c.len_utf8()..])
                    .unwrap_or_else(String::new)
            ),
            name: input_node.name.clone(),
            description: input_node.description.clone(),
            out_of_scope: input_node.out_of_scope.clone(),
            reason_out_of_scope: Some("".to_string()),
            has_open_threats: false,
            is_bidirectional: None,
            is_encrypted: None,
            is_public_network: None,
            protocol: None,
            handles_card_payment: None,
            handles_goods_or_services: None,
            is_web_application: None,
            privilege_level: None,
            threats: input_node
                .threats
                .iter()
                .map(|input_threat_name| {
                    threats
                        .iter()
                        .filter(|threat| threat.title == *input_threat_name)
                        .last()
                })
                .filter(|input_threat| input_threat.is_some())
                .map(|input_threat| Threat::from_input_diagram(&input_threat.unwrap(), &config))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Threat {
    id: String,
    title: String,
    status: String,
    severity: String,
    #[serde(rename = "type")]
    type_field: String,
    description: String,
    mitigation: String,
    model_type: String,
    new: bool,
    number: u32,
    score: String,
}

impl MappingFromInputThreat for Threat {
    fn from_input_diagram(input_threat: &threat::Threat, config: &Config) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: input_threat.title.clone(),
            status: input_threat.status.to_string(),
            severity: input_threat.severity.to_string(),
            type_field: input_threat.type_field.to_string(),
            description: input_threat.description.clone(),
            mitigation: input_threat.mitigation.clone(),
            model_type: "STRIDE".to_string(),
            new: false,
            number: 1,
            score: "".to_string(),
        }
    }
}
