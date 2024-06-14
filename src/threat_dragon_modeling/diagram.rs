use std::{collections::HashMap, f64::consts::PI};

use ordered_float::OrderedFloat;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::config::Config,
    input::{
        input_diagram::{InputDiagram, Node, TypeNode},
        threat::Threat,
    },
    process::process::{
        MappingFromInputDiagram, MappingFromInputNode, MappingFromInputNodeAndThreats,
    },
};

use super::cell_data::CellData;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Diagram {
    pub id: u32,
    pub title: String,
    pub diagram_type: String,
    pub placeholder: String,
    pub thumbnail: String,
    pub version: String,
    pub cells: Vec<Cell>,
}

impl MappingFromInputDiagram for Diagram {
    fn from_input_diagram(
        index: usize,
        input_diagram: &InputDiagram,
        config: &Config,
        threats: &Vec<Threat>,
    ) -> Self {
        let mut cells = input_diagram
            .nodes
            .iter()
            .map(|input_node| Cell::from_input_diagram(&input_node, &config, threats))
            .collect();
        Self::update_source_and_destination(&mut cells, &input_diagram);
        Self::update_cells_position(&mut cells, &input_diagram);
        Self::add_trust_boundaries(&mut cells, input_diagram);

        Self {
            id: index as u32,
            title: input_diagram.title.clone(),
            diagram_type: "STRIDE".to_string(),
            placeholder: input_diagram.description.clone(),
            thumbnail: "./public/content/images/thumbnail.stride.jpg".to_string(),
            version: config.threat_dragon_version.clone(),
            cells,
        }
    }
}

impl Diagram {
    fn update_source_and_destination(cells: &mut Vec<Cell>, input_diagram: &InputDiagram) {
        let mut node_ids_map = HashMap::new();
        cells.iter().for_each(|cell| {
            node_ids_map.insert(cell.data.name.clone(), cell.id.clone());
        });
        input_diagram
            .nodes
            .iter()
            .filter(|node| node.type_node == TypeNode::Flow)
            .for_each(|node_flow| {
                cells
                    .iter_mut()
                    .filter(|cell| cell.data.name.eq_ignore_ascii_case(&node_flow.name))
                    .for_each(|cell| {
                        if let Some(node_flow_source) = &node_flow.source {
                            if let Some(id) = node_ids_map.get(node_flow_source) {
                                cell.source = Some(Endpoint { cell: id.clone() })
                            }
                        }
                        if let Some(node_flow_dest) = &node_flow.destination {
                            if let Some(id) = node_ids_map.get(node_flow_dest) {
                                cell.target = Some(Endpoint { cell: id.clone() })
                            }
                        }
                        cell.labels = Some(vec![node_flow.name.clone()])
                    });
            });
    }

    fn update_cells_position(cells: &mut Vec<Cell>, input_diagram: &InputDiagram) {
        let mut node_region = HashMap::new();
        let mut regions_positions: HashMap<String, (Position, Vec<String>)> = HashMap::new();
        input_diagram.nodes.iter().for_each(|node| {
            let default_region_out_of_scope = "South".to_string();
            let default_region = "Center".to_string();
            let region = if let Some(out_of_scope) = node.out_of_scope {
                if out_of_scope {
                    if let Some(trust_boundary) = node.trust_boundary.clone() {
                        trust_boundary
                    } else {
                        default_region_out_of_scope
                    }
                } else {
                    default_region
                }
            } else {
                default_region
            };
            if let Some((pos, node_list)) = regions_positions.get(&region) {
                let mut new_node_list_in_region = node_list.clone();
                new_node_list_in_region.push(node.name.clone());
                let node_list_size_in_region = new_node_list_in_region.len();
                regions_positions.insert(
                    region.clone(),
                    (Position { x: 500., y: 520. }, new_node_list_in_region),
                );
                node_region.insert(
                    node.name.clone(),
                    (region.clone(), node_list_size_in_region),
                );
            } else {
                regions_positions.insert(
                    region.clone(),
                    (Position { x: 500., y: 500. }, vec![node.name.clone()]),
                );
                node_region.insert(node.name.clone(), (region.clone(), 1));
            }
        });
        // Dimensions de l'espace total
        let radius = 450.0;
        let region_radius = 120.;
        let center_x = 500.0;
        let center_y = 500.0;
        // Initialiser un générateur de nombres aléatoires
        let mut rng = rand::thread_rng();
        // Calculer les positions des régions en cercle
        let region_count = regions_positions.len();
        regions_positions
            .iter_mut()
            .filter(|(region, _)| region.ne(&&"Center".to_string()))
            .enumerate()
            .for_each(|(index, (region, (pos, _)))| {
                let angle = 2.0 * PI * (index as f64) / (region_count as f64);
                let region_center_x = center_x + radius * angle.cos();
                let region_center_y = center_y + radius * angle.sin();
                pos.x = region_center_x;
                pos.y = region_center_y;
            });

        cells
            .iter_mut()
            .filter(|cell| cell.shape != "flow")
            .for_each(|cell| {
                let (region, index_in_region) =
                    node_region.get(&cell.data.name).unwrap().to_owned();
                let (region_position, nodes) = regions_positions.get(&region).unwrap().to_owned();
                let angle = 2.0 * PI * (index_in_region as f64) / (nodes.len() as f64);
                let test = if index_in_region > 2 { 0 } else { 0 };
                let dx = region_radius * angle.cos();
                let dy = region_radius * angle.sin();
                cell.position = Some(Position {
                    x: region_position.x + dx + test as f64,
                    y: region_position.y + dy + test as f64,
                });
                cell.size = Some(Size {
                    width: 140,
                    height: 140,
                })
            });
    }

    fn add_trust_boundaries(cells: &mut Vec<Cell>, input_diagram: &InputDiagram) {
        let mut trust_boundaries: HashMap<String, Vec<Node>> = HashMap::new();

        input_diagram
            .nodes
            .iter()
            .filter(|input_node| input_node.trust_boundary.is_some())
            .for_each(|input_node| {
                if let Some(node_in_trust_boundary) =
                    trust_boundaries.get(&input_node.clone().trust_boundary.clone().unwrap())
                {
                    let mut new_node_in_trust_boundary = node_in_trust_boundary.clone();
                    new_node_in_trust_boundary.push(input_node.clone());
                    trust_boundaries.insert(
                        input_node.trust_boundary.clone().unwrap(),
                        new_node_in_trust_boundary,
                    );
                } else {
                    trust_boundaries.insert(
                        input_node.trust_boundary.clone().unwrap(),
                        vec![input_node.clone()],
                    );
                }
            });

        trust_boundaries
            .iter()
            .for_each(|(trust_boundary, node_list)| {
                let cell_last_node = cells
                    .iter()
                    .filter(|cell| cell.data.name == node_list.get(0).unwrap().name)
                    .last();

                let cells_for_node_list = cells.iter().filter(|cell| {
                    node_list
                        .iter()
                        .map(|node| node.name.clone())
                        .collect::<Vec<String>>()
                        .contains(&cell.data.name)
                });

                let min_pos_x: Option<f64> = cells_for_node_list
                    .clone()
                    .map(|cell| {
                        if let Some(pos) = &cell.position {
                            pos.x
                        } else {
                            -1.
                        }
                    })
                    .filter(|pos_x| *pos_x != -1.)
                    .map(OrderedFloat)
                    .min()
                    .map(|ordered| ordered.into_inner());
                let min_pos_y: Option<f64> = cells_for_node_list
                    .clone()
                    .map(|cell| {
                        if let Some(pos) = &cell.position {
                            pos.y
                        } else {
                            -1.
                        }
                    })
                    .filter(|pos_x| *pos_x != -1.)
                    .map(OrderedFloat)
                    .min()
                    .map(|ordered| ordered.into_inner());
                let max_pos_x: Option<f64> = cells_for_node_list
                    .clone()
                    .map(|cell| {
                        if let Some(pos) = &cell.position {
                            pos.x
                        } else {
                            -1.
                        }
                    })
                    .filter(|pos_x| *pos_x != -1.)
                    .map(OrderedFloat)
                    .max()
                    .map(|ordered| ordered.into_inner());
                let max_pos_y: Option<f64> = cells_for_node_list
                    .clone()
                    .map(|cell| {
                        if let Some(pos) = &cell.position {
                            pos.y
                        } else {
                            -1.
                        }
                    })
                    .filter(|pos_x| *pos_x != -1.)
                    .map(OrderedFloat)
                    .max()
                    .map(|ordered| ordered.into_inner());

                let max_size_width: Option<u32> = cells_for_node_list
                    .clone()
                    .map(|cell| {
                        if let Some(size) = &cell.size {
                            size.width
                        } else {
                            0
                        }
                    })
                    .filter(|size_x| *size_x > 0)
                    .max();
                let max_size_height: Option<u32> = cells_for_node_list
                    .clone()
                    .map(|cell| {
                        if let Some(size) = &cell.size {
                            size.height
                        } else {
                            0
                        }
                    })
                    .filter(|size_x| *size_x > 0)
                    .max();
                let margin: u32 = 40;
                if let Some(cell_last_node) = cell_last_node {
                    cells.push(Cell {
                        position: Some(Position {
                            x: min_pos_x.unwrap() - margin as f64,
                            y: min_pos_y.unwrap() - margin as f64,
                        }),
                        size: Some(Size {
                            width: (max_pos_x.unwrap() - min_pos_x.unwrap()) as u32
                                + max_size_width.unwrap()
                                + margin * 2,
                            height: (max_pos_y.unwrap() - min_pos_y.unwrap()) as u32
                                + max_size_height.unwrap()
                                + margin * 2,
                        }),
                        attrs: Some(Attrs {
                            header_text: Some(Text {
                                text: trust_boundary.clone(),
                            }),
                            text: None,
                            body: None,
                            line: None,
                        }),
                        visible: None,
                        shape: "trust-boundary-box".to_string(),
                        id: Uuid::new_v4().to_string(),
                        z_index: 0,
                        connector: None,
                        data: CellData {
                            type_field: "tm.BoundaryBox".to_string(),
                            name: trust_boundary.clone(),
                            description: "".to_string(),
                            out_of_scope: None,
                            reason_out_of_scope: None,
                            has_open_threats: false,
                            is_bidirectional: None,
                            is_encrypted: None,
                            is_public_network: None,
                            protocol: None,
                            handles_card_payment: None,
                            handles_goods_or_services: None,
                            is_web_application: None,
                            privilege_level: None,
                            threats: [].to_vec(),
                        },
                        source: None,
                        target: None,
                        labels: None,
                    })
                }
            });
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Attrs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    pub shape: String,
    pub id: String,
    pub z_index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<String>,
    pub data: CellData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Endpoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Endpoint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

impl MappingFromInputNodeAndThreats for Cell {
    fn from_input_diagram(input_node: &Node, config: &Config, threats: &Vec<Threat>) -> Self {
        Self {
            position: None,
            size: None,
            attrs: Some(Attrs::from_input_diagram(&input_node, &config)),
            visible: None,
            shape: input_node.type_node.to_string(),
            id: Uuid::new_v4().to_string(),
            z_index: 1,
            connector: None,
            data: CellData::from_input_diagram(&input_node, &config, threats),
            source: None,
            target: None,
            labels: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Size {
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Attrs {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Body>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
}

impl MappingFromInputNode for Attrs {
    fn from_input_diagram(input_node: &Node, config: &Config) -> Self {
        let (header_text, text, body, line) = match input_node.type_node {
            TypeNode::Process => (
                None,
                Some(Text::from_input_diagram(&input_node, &config)),
                Some(Body::from_input_diagram(&input_node, &config)),
                None,
            ),
            TypeNode::Flow => (
                None,
                None,
                None,
                Some(Line::from_input_diagram(&input_node, &config)),
            ),
        };

        Self {
            header_text,
            text,
            body,
            line,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Text {
    text: String,
}

impl MappingFromInputNode for Text {
    fn from_input_diagram(input_node: &Node, config: &Config) -> Self {
        Self {
            text: input_node.name.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Body {
    stroke: String,
    stroke_width: f64,
    stroke_dasharray: Option<String>,
}

impl MappingFromInputNode for Body {
    fn from_input_diagram(input_node: &Node, config: &Config) -> Self {
        let (stroke, stroke_width) = if input_node.threats.len() > 0 {
            ("red".to_string(), 1.5)
        } else {
            ("#333333".to_string(), 3.)
        };

        let stroke_dasharray = if let Some(out_of_scope) = input_node.out_of_scope {
            if out_of_scope {
                Some("4 3".to_string())
            } else {
                None
            }
        } else {
            None
        };
        Self {
            stroke,
            stroke_width,
            stroke_dasharray,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Line {
    stroke: String,
    stroke_width: f64,
    source_marker: SourceMarker,
    target_marker: TargetMarker,
    stroke_dasharray: Option<String>,
}

impl MappingFromInputNode for Line {
    fn from_input_diagram(input_node: &Node, config: &Config) -> Self {
        let (stroke, stroke_width) = if input_node.threats.len() > 0 {
            ("red".to_string(), 1.5)
        } else {
            ("#333333".to_string(), 3.)
        };
        Self {
            stroke,
            stroke_width,
            source_marker: SourceMarker {
                name: "".to_string(),
            },
            target_marker: TargetMarker {
                name: "block".to_string(),
            },
            stroke_dasharray: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SourceMarker {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct TargetMarker {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Endpoint {
    cell: String,
}
