use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::threat::Threat;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputDiagram {
    pub title: String,
    pub description: String,
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub name: String,
    #[serde(rename = "type")]
    pub type_node: TypeNode,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_scope: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_boundary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    pub threats: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TypeNode {
    Process,
    Flow,
}

impl Display for TypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TypeNode::Process => write!(f, "process"),
            TypeNode::Flow => write!(f, "flow"),
        }
    }
}
