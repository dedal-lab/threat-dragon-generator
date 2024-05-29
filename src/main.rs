use std::{
    env,
    fs::{self, File},
    io::{BufReader, BufWriter},
};

use input::threat::Threat;
use serde::{Deserialize, Serialize};

use crate::{
    config::config::{Config, ConfigYaml},
    input::input_diagram::InputDiagram,
    threat_dragon_modeling::general::ThreatModeling,
};

mod config;
mod input;
mod process;
mod threat_dragon_modeling;

fn main() {
    let config_path = env::var("CONFIG_PATH");
    let threat_path = env::var("THREAT_PATH");
    let diagram_path = env::var("DIAGRAM_PATH");
    let output_path = env::var("OUTPUT_PATH");

    let file = File::open(config_path.unwrap()).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).unwrap();
    println!("{:?}", config);

    let file = File::open(threat_path.unwrap()).unwrap();
    let reader = BufReader::new(file);
    let threat_list: Vec<Threat> = serde_yaml::from_reader(reader).unwrap();
    println!("{:?}", threat_list);

    let entries = fs::read_dir(&diagram_path.unwrap())
        .unwrap()
        .filter_map(Result::ok) // Filtrer les erreurs
        .filter(|e| e.path().is_file()) // Filtrer uniquement les fichiers
        .collect::<Vec<_>>();

    let mut threat_model_diagram_list: Vec<InputDiagram> = Vec::new();
    for entry in entries {
        let content = fs::read_to_string(&entry.path()).unwrap();
        let threat_model_yaml: InputDiagram = serde_yaml::from_str(&content).unwrap();
        println!("{:#?}", threat_model_yaml);
        threat_model_diagram_list.push(threat_model_yaml);
    }

    let json_model_output = output_path.unwrap();
    let new_threat_modeling =
        ThreatModeling::new(&threat_model_diagram_list, &config, &threat_list);
    let output_file = File::create(json_model_output).unwrap();
    let writer = BufWriter::new(output_file);
    serde_json::to_writer_pretty(writer, &new_threat_modeling).unwrap();
}
