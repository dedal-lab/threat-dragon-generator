use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::config::config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub trust_level: Option<String>,
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

impl InputDiagram {
    pub fn create_child_diagrams(&self, config: &Config) -> Vec<Self> {
        let mut child_list = Vec::new();

        config
            .diagrams
            .iter()
            .filter(|config_diagram| self.title == config_diagram.parent)
            .for_each(|config_diagram| {
                let mut nodes: Vec<Node> = self
                    .nodes
                    .iter()
                    .filter(|node_process| config_diagram.nodes.contains(&node_process.name))
                    .map(|node| node.clone())
                    .collect();
                let nodes_flows: Vec<Node> = self
                    .nodes
                    .iter()
                    .filter(|node| {
                        if let Some(node_source) = node.source.clone() {
                            if let Some(node_dest) = node.destination.clone() {
                                if config_diagram.nodes.contains(&node_source)
                                    && config_diagram.nodes.contains(&node_dest)
                                {
                                    true
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .map(|node| node.clone())
                    .collect();
                nodes.append(&mut nodes_flows.clone());
                let child_diagram = InputDiagram {
                    title: config_diagram.name.clone(),
                    description: config_diagram.description.clone(),
                    nodes,
                };
                child_list.push(child_diagram);
            });
        child_list
    }
}
