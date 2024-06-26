use std::{collections::HashMap, path::Path};

use rust_xlsxwriter::{Format, Table, TableColumn, TableStyle, Workbook, Worksheet};

use crate::{
    config::config::{Asset, Config, TrustBoundary},
    input::{
        input_diagram::{InputDiagram, TypeNode},
        threat::Threat,
    },
};

use super::excel_error::ExcelError;

pub fn create_reports(
    output_folder: &Path,
    input_diagram: &InputDiagram,
    config: &Config,
    threats: &Vec<Threat>,
) -> Result<(), ExcelError> {
    let mut workbook = Workbook::new();

    create_software_worksheet(&input_diagram, &mut workbook)?;
    create_entry_points_worksheet(&input_diagram, &mut workbook)?;
    create_trust_boundary_worksheet(&input_diagram, &config, &mut workbook)?;
    create_assets_worksheet(&input_diagram, &config, &mut workbook)?;
    create_threats_worksheet(&input_diagram, &threats, &mut workbook)?;
    create_vectors_worksheet(&input_diagram, &threats, &mut workbook)?;
    let mut workbook_save_path = output_folder.join(&input_diagram.title);
    workbook_save_path.set_extension("xlsx");
    // Save the file to disk.
    workbook
        .save(workbook_save_path)
        .map_err(|e| ExcelError::Save(format!("{}", e)))?;

    Ok(())
}

fn create_table(headers: &Vec<String>, data: &Vec<Vec<String>>, worksheet: &mut Worksheet) {
    // Create a new table and configure it.
    let data_size = data.len();
    let titles: Vec<TableColumn> = headers
        .iter()
        .map(|header| TableColumn::new().set_header(header))
        .collect();
    worksheet.write_row_matrix(1, 0, data).unwrap();
    let table = Table::new()
        .set_banded_rows(true)
        .set_style(TableStyle::Medium23)
        .set_columns(&titles)
        .set_total_row(false);

    worksheet
        .add_table(0, 0, (data_size) as u32, (titles.len() - 1) as u16, &table)
        .unwrap();
}

fn create_entry_points_worksheet(
    input_diagram: &InputDiagram,
    workbook: &mut Workbook,
) -> Result<(), ExcelError> {
    // Add a worksheet to the workbook.
    let entry_point_worksheet = workbook.add_worksheet();

    entry_point_worksheet
        .set_name("EntryPoint")
        .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    let column_titles = vec![
        "ID".to_string(),
        "Direction".to_string(),
        "Description".to_string(),
        "Asset Name".to_string(),
        "Trust Level".to_string(),
        "Microservice".to_string(),
    ];

    let mut data: Vec<Vec<String>> = Vec::new();

    input_diagram
        .nodes
        .iter()
        .filter(|node| node.type_node == TypeNode::Flow)
        .for_each(|node_flow| {
            let node_source = input_diagram
                .nodes
                .iter()
                .filter(|node| node.name == node_flow.source.clone().unwrap())
                .last();
            let node_dest = input_diagram
                .nodes
                .iter()
                .filter(|node| node.name == node_flow.destination.clone().unwrap())
                .last();
            let (microservice, direction) = if !node_source.unwrap().out_of_scope.unwrap() {
                (node_source.unwrap().name.clone(), "Exit".to_string())
            } else if !node_dest.unwrap().out_of_scope.unwrap() {
                (node_dest.unwrap().name.clone(), "Entry".to_string())
            } else {
                ("Unknown".to_string(), "".to_string())
            };
            let trust_level = match &node_flow.trust_level {
                Some(trust_level) => trust_level.clone(),
                None => "Unknown".to_string(),
            };
            data.push(vec![
                node_flow.name.clone(),
                direction,
                node_flow.description.clone(),
                node_flow.asset.clone().unwrap_or("".to_string()),
                trust_level,
                microservice,
            ]);
        });
    create_table(&column_titles, &data, entry_point_worksheet);
    entry_point_worksheet.autofit();
    let format_text_wrap = Format::new().set_text_wrap();
    entry_point_worksheet
        .set_column_width(2, 40)
        .map_err(|e| ExcelError::SetColumnWidth(format!("{}", e)))?;
    entry_point_worksheet
        .set_column_format(2, &format_text_wrap)
        .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;

    Ok(())
}

fn create_trust_boundary_worksheet(
    input_diagram: &InputDiagram,
    config: &Config,
    workbook: &mut Workbook,
) -> Result<(), ExcelError> {
    // Add a worksheet to the workbook.
    let entry_point_worksheet = workbook.add_worksheet();

    entry_point_worksheet
        .set_name("Trust Boundaries")
        .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    let mut trust_boundaries_map: HashMap<String, TrustBoundary> = HashMap::new();

    input_diagram
        .nodes
        .iter()
        .filter(|node| node.trust_boundary.is_some())
        .for_each(|node_with_trust_boundary| {
            config
                .trust_boundaries
                .iter()
                .filter(|tb| tb.name == node_with_trust_boundary.trust_boundary.clone().unwrap())
                .for_each(|tb| {
                    trust_boundaries_map.insert(
                        node_with_trust_boundary.trust_boundary.clone().unwrap(),
                        tb.clone(),
                    );
                });
        });

    let mut data: Vec<Vec<String>> = Vec::new();
    let mut data_size = 0;
    let headers = vec![
        "ID".to_string(),
        "Description".to_string(),
        "Limit of Access".to_string(),
        "Level of Authorization".to_string(),
    ];
    trust_boundaries_map
        .iter()
        .enumerate()
        .for_each(|(index, (tb_name, tb))| {
            data.push(vec![
                tb_name.clone().to_string(),
                tb.description.clone().to_string(),
                tb.limit_of_access.clone().to_string(),
                tb.level_of_authorization.clone().to_string(),
            ]);
            data_size = index + 1;
        });
    create_table(&headers, &data, entry_point_worksheet);
    entry_point_worksheet.autofit();
    let format_text_wrap = Format::new().set_text_wrap();
    entry_point_worksheet
        .set_column_width(1, 40)
        .map_err(|e| ExcelError::SetColumnWidth(format!("{}", e)))?;
    entry_point_worksheet
        .set_column_format(1, &format_text_wrap)
        .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;

    Ok(())
}

fn create_threats_worksheet(
    input_diagram: &InputDiagram,
    threats: &Vec<Threat>,
    workbook: &mut Workbook,
) -> Result<(), ExcelError> {
    // Add a worksheet to the workbook.
    let threats_worksheet = workbook.add_worksheet();

    threats_worksheet
        .set_name("Threats")
        .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    let column_titles = vec![
        "ID".to_string(),
        "Type".to_string(),
        "STRIDE".to_string(),
        "Description".to_string(),
        "Vector".to_string(),
        "Status".to_string(),
        "Mitigations".to_string(),
    ];

    let mut data: Vec<Vec<String>> = Vec::new();

    input_diagram.nodes.iter().for_each(|node| {
        node.threats.iter().for_each(|threat_str| {
            let threat = threats
                .iter()
                .filter(|threat| threat.title == *threat_str)
                .last();
            if let Some(threat) = threat {
                data.push(vec![
                    node.name.clone().to_string(),
                    node.type_node.clone().to_string(),
                    threat.type_field.clone().to_string(),
                    threat.description.clone().to_string(),
                    threat.vector.clone().to_string(),
                    threat.status.clone().to_string(),
                    threat.mitigation.clone().to_string(),
                ]);
            }
        });
    });

    let format_text_wrap = Format::new().set_text_wrap();
    create_table(&column_titles, &data, threats_worksheet);
    threats_worksheet.autofit();
    threats_worksheet
        .set_column_width(3, 40)
        .map_err(|e| ExcelError::SetColumnWidth(format!("{}", e)))?;
    threats_worksheet
        .set_column_format(3, &format_text_wrap)
        .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;
    threats_worksheet
        .set_column_width(6, 40)
        .map_err(|e| ExcelError::SetColumnWidth(format!("{}", e)))?;
    threats_worksheet
        .set_column_format(6, &format_text_wrap)
        .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;
    Ok(())
}

fn create_software_worksheet(
    input_diagram: &InputDiagram,
    workbook: &mut Workbook,
) -> Result<(), ExcelError> {
    // Add a worksheet to the workbook.
    let software_worksheet = workbook.add_worksheet();

    software_worksheet
        .set_name("Software Component")
        .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    let column_titles = vec![
        "Name".to_string(),
        "Description".to_string(),
        "Trust Level".to_string(),
        "Out of scope".to_string(),
    ];

    let mut data: Vec<Vec<String>> = Vec::new();

    input_diagram
        .nodes
        .iter()
        .filter(|node| node.type_node == TypeNode::Process)
        .for_each(|node_process| {
            let mut row: Vec<String> = Vec::new();
            let out_of_scope = if let Some(out_of_scope) = node_process.out_of_scope.clone() {
                if out_of_scope {
                    "Yes".to_string()
                } else {
                    "No".to_string()
                }
            } else {
                "".to_string()
            };
            row.push(node_process.name.clone());
            row.push(node_process.description.clone());
            row.push(node_process.trust_level.clone().unwrap_or("".to_string()));
            row.push(out_of_scope);
            data.push(row);
        });
    create_table(&column_titles, &data, software_worksheet);
    software_worksheet.autofit();
    let format_text_wrap = Format::new().set_text_wrap();
    software_worksheet
        .set_column_width(1, 40)
        .map_err(|e| ExcelError::SetColumnWidth(format!("{}", e)))?;
    software_worksheet
        .set_column_format(1, &format_text_wrap)
        .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;
    Ok(())
}

fn create_assets_worksheet(
    input_diagram: &InputDiagram,
    config: &Config,
    workbook: &mut Workbook,
) -> Result<(), ExcelError> {
    // Add a worksheet to the workbook.
    let asset_worksheet = workbook.add_worksheet();

    asset_worksheet
        .set_name("Asset worksheet")
        .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    let column_titles = vec!["Name".to_string(), "Description".to_string()];

    let mut asset_map: HashMap<String, Asset> = HashMap::new();

    input_diagram
        .nodes
        .iter()
        .filter(|node| node.type_node == TypeNode::Flow)
        .for_each(|node_flow| {
            if let Some(node_flow_asset) = node_flow.asset.clone() {
                let asset = config
                    .assets
                    .iter()
                    .filter(|asset_config| asset_config.name == node_flow_asset)
                    .last();

                if let Some(asset) = asset {
                    asset_map.insert(node_flow_asset, asset.clone());
                }
            }
        });
    let mut data: Vec<Vec<String>> = Vec::new();

    asset_map.iter().for_each(|(_key, value)| {
        let mut row: Vec<String> = Vec::new();
        row.push(value.name.clone());
        row.push(value.description.clone());
        data.push(row);
    });

    create_table(&column_titles, &data, asset_worksheet);
    asset_worksheet.autofit();
    let format_text_wrap = Format::new().set_text_wrap();
    asset_worksheet
        .set_column_width(1, 40)
        .map_err(|e| ExcelError::SetColumnWidth(format!("{}", e)))?;
    asset_worksheet
        .set_column_format(1, &format_text_wrap)
        .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;
    Ok(())
}

fn create_vectors_worksheet(
    input_diagram: &InputDiagram,
    threats: &Vec<Threat>,
    workbook: &mut Workbook,
) -> Result<(), ExcelError> {
    let vectors_worksheet = workbook.add_worksheet();

    vectors_worksheet
        .set_name("Vectors")
        .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    let column_titles = vec!["Name".to_string()];

    let mut vector_map: HashMap<String, String> = HashMap::new();

    input_diagram.nodes.iter().for_each(|node| {
        node.threats.iter().for_each(|threat_str| {
            let threat = threats
                .iter()
                .filter(|threat| threat.title == *threat_str)
                .last();
            if let Some(threat) = threat {
                vector_map.insert(threat.vector.clone(), threat.vector.clone());
            }
        });
    });

    let vectorl_list: Vec<Vec<String>> = vector_map
        .iter()
        .map(|(index, _value)| vec![index.clone()])
        .collect();

    create_table(&column_titles, &vectorl_list, vectors_worksheet);
    vectors_worksheet.autofit();

    Ok(())
}
