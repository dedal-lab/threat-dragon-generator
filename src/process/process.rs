use crate::{
    config::config::Config,
    input::{
        input_diagram::{InputDiagram, Node},
        threat::Threat,
    },
};

pub trait MappingFromInputDiagram {
    fn from_input_diagram(input_diagram: &InputDiagram, config: &Config) -> Self;
}

pub trait MappingFromVecInputDiagram {
    fn from_input_diagram(input_diagram: &Vec<InputDiagram>, config: &Config) -> Self;
}

pub trait MappingFromInputNode {
    fn from_input_diagram(input_node: &Node, config: &Config) -> Self;
}

pub trait MappingFromInputThreat {
    fn from_input_diagram(input_threat: &Threat, config: &Config) -> Self;
}
