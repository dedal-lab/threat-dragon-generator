use std::{
    env,
    fs::{self, File},
    io::{BufReader, BufWriter, ErrorKind},
    path::{self, Path},
    result,
};

use input::threat::Threat;

use crate::{
    config::config::Config, input::input_diagram::InputDiagram,
    threat_dragon_modeling::general::ThreatModeling,
};

use reports::xls::xls_reports;

mod config;
mod input;
mod process;
mod reports;
mod threat_dragon_modeling;

fn main() {
    let config_path = env::var("CONFIG_PATH");
    let threat_path = env::var("THREAT_PATH");
    let diagram_path = env::var("DIAGRAM_PATH");
    let output_path = env::var("OUTPUT_PATH");

    let file = File::open(config_path.unwrap()).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader).unwrap();
    // println!("{:?}", config);

    let file = File::open(threat_path.unwrap()).unwrap();
    let reader = BufReader::new(file);
    let threat_list: Vec<Threat> = serde_yaml::from_reader(reader).unwrap();
    // println!("{:?}", threat_list);

    let entries = fs::read_dir(&diagram_path.unwrap())
        .unwrap()
        .filter_map(Result::ok) // Filtrer les erreurs
        .filter(|e| e.path().is_file()) // Filtrer uniquement les fichiers
        .collect::<Vec<_>>();

    let mut threat_model_diagram_list: Vec<InputDiagram> = Vec::new();
    for entry in entries {
        let content = fs::read_to_string(&entry.path()).unwrap();
        let threat_model_yaml: InputDiagram = serde_yaml::from_str(&content).unwrap();
        // println!("{:#?}", threat_model_yaml);
        let childs = threat_model_yaml.create_child_diagrams(&config);
        threat_model_diagram_list.push(threat_model_yaml);
        childs
            .iter()
            .for_each(|child| threat_model_diagram_list.push(child.clone()));
        // println!("{:#?}", childs);
    }

    let model_output = output_path.unwrap();
    let new_threat_modeling =
        ThreatModeling::new(&threat_model_diagram_list, &config, &threat_list);

    let output_folder_path = Path::new(&model_output);
    let result = fs::create_dir_all(output_folder_path);
    if let Err(result) = result {
        if result.kind() != ErrorKind::AlreadyExists {
            panic!("{}", result);
        }
    }

    if let Some(directory) = output_folder_path.file_name() {
        if let Some(directory_str) = directory.to_str() {
            let mut json_model_fullpath = output_folder_path.join(directory_str);
            json_model_fullpath.set_extension("json");
            let json_model_file = File::create(json_model_fullpath).unwrap();
            let writer = BufWriter::new(json_model_file);
            serde_json::to_writer_pretty(writer, &new_threat_modeling).unwrap();

            xls_reports::create_reports(&threat_model_diagram_list.get(0).unwrap(), &config)
                .unwrap();
        }
    };
}
