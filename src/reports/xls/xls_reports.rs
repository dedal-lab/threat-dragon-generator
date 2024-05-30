use std::collections::HashMap;

use rust_xlsxwriter::{Table, TableColumn, TableStyle, Workbook, Worksheet};

use crate::{
    config::config::{Config, TrustBoundary},
    input::input_diagram::{InputDiagram, TypeNode},
};

use super::excel_error::ExcelError;

pub fn create_reports(input_diagram: &InputDiagram, config: &Config) -> Result<(), ExcelError> {
    let mut workbook = Workbook::new();

    create_entry_points_worksheet(&input_diagram, &mut workbook)?;
    create_trust_boundary_worksheet(&input_diagram, &config, &mut workbook)?;

    // Save the file to disk.
    workbook
        .save("demo.xlsx")
        .map_err(|e| ExcelError::Save(format!("{}", e)))?;

    Ok(())
}

pub fn create_entry_points_worksheet(
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
        "Name".to_string(),
        "Description".to_string(),
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
            let microservice = if !node_source.unwrap().out_of_scope.unwrap() {
                node_source.unwrap().name.clone()
            } else if !node_dest.unwrap().out_of_scope.unwrap() {
                node_dest.unwrap().name.clone()
            } else {
                "Unknown".to_string()
            };

            data.push(vec![
                node_flow.name.clone(),
                node_flow.name.clone(),
                node_flow.description.clone(),
                "Unknown".to_string(),
                microservice,
            ]);
        });
    create_table(&column_titles, &data, entry_point_worksheet);
    entry_point_worksheet.autofit();

    Ok(())
}

pub fn create_trust_boundary_worksheet(
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
