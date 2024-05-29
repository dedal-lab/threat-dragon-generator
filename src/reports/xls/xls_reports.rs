use std::collections::HashMap;

use rust_xlsxwriter::{
    Color, ExcelDateTime, Format, FormatAlign, FormatBorder, Formula, Table, TableColumn,
    TableStyle, Url, Workbook, Worksheet,
};

use crate::{
    config::config::{Config, TrustBoundary},
    input::input_diagram::{InputDiagram, TypeNode},
};

use super::excel_error::ExcelError;

pub fn create_reports(input_diagram: &InputDiagram, config: &Config) -> Result<(), ExcelError> {
    let bold_format = Format::new().set_bold();
    let decimal_format = Format::new().set_num_format("0.000");
    let date_format = Format::new().set_num_format("yyyy-mm-dd");
    let merge_format = Format::new()
        .set_border(FormatBorder::Double)
        .set_align(FormatAlign::Center)
        .set_bold();

    let mut workbook = Workbook::new();

    create_entry_points_worksheet(&input_diagram, &mut workbook)?;
    create_trust_boundary_worksheet(&input_diagram, &config, &mut workbook)?;
    // let worksheet = workbook.worksheet_from_index(0).unwrap();

    // // Set the column width for clarity.
    // worksheet
    //     .set_column_width(0, 22)
    //     .map_err(|e| ExcelError::SetColumnWidth(format!("{}", e)))?;

    // worksheet
    //     .set_name("Ceci est un test")
    //     .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    // // Write a string without formatting.
    // worksheet
    //     .write(0, 0, "Hello")
    //     .map_err(|e| ExcelError::Write(format!("{}", e)))?;

    // // Write a string with the bold format defined above.
    // worksheet
    //     .write_with_format(1, 0, "World", &bold_format)
    //     .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;

    // // Write some numbers.
    // worksheet
    //     .write(2, 0, 1)
    //     .map_err(|e| ExcelError::Write(format!("{}", e)))?;
    // worksheet
    //     .write(3, 0, 2.34)
    //     .map_err(|e| ExcelError::Write(format!("{}", e)))?;

    // // Write a number with formatting.
    // worksheet
    //     .write_with_format(4, 0, 3.00, &decimal_format)
    //     .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;

    // // Write a formula.
    // worksheet
    //     .write(5, 0, Formula::new("=SIN(PI()/4)"))
    //     .map_err(|e| ExcelError::Write(format!("{}", e)))?;

    // // Write a date.
    // let date = ExcelDateTime::from_ymd(2023, 1, 25)
    //     .map_err(|e| ExcelError::ExcelDateTime(format!("{}", e)))?;
    // worksheet
    //     .write_with_format(6, 0, &date, &date_format)
    //     .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;

    // // Write some links.
    // worksheet
    //     .write(7, 0, Url::new("https://www.rust-lang.org"))
    //     .map_err(|e| ExcelError::Write(format!("{}", e)))?;
    // worksheet
    //     .write(8, 0, Url::new("https://www.rust-lang.org").set_text("Rust"))
    //     .map_err(|e| ExcelError::Write(format!("{}", e)))?;

    // // Write some merged cells.
    // worksheet
    //     .merge_range(9, 0, 9, 1, "Merged cells", &merge_format)
    //     .map_err(|e| ExcelError::MergeRange(format!("{}", e)))?;

    // // Insert an image.
    // let image = Image::new("test.png").map_err(|e| ExcelError::NewImage(format!("{}", e)))?;
    // worksheet
    //     .insert_image(1, 2, &image)
    //     .map_err(|e| ExcelError::InsertImage(format!("{}", e)))?;

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
    // Create some formats to use in the worksheet.
    let title_format = Format::new()
        .set_border(FormatBorder::Double)
        .set_align(FormatAlign::Center)
        .set_bold()
        .set_background_color(Color::Gray);
    let row_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left)
        .set_background_color(Color::White);

    // Add a worksheet to the workbook.
    let entry_point_worksheet = workbook.add_worksheet();

    entry_point_worksheet
        .set_name("EntryPoint")
        .map_err(|e| ExcelError::SetName(format!("{}", e)))?;

    let column_titles = ["ID", "Name", "Description", "Trust Level", "Microservice"];
    entry_point_worksheet
        .write_row_with_format(0, 0, column_titles, &title_format)
        .map_err(|e| ExcelError::WriteWithFormat(format!("{}", e)))?;

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

            let row_values = [
                node_flow.name.clone(),
                node_flow.name.clone(),
                node_flow.description.clone(),
                "Unknown".to_string(),
                microservice,
            ];
            entry_point_worksheet
                .write_row_with_format(1, 0, row_values, &row_format)
                .unwrap();
        });

    entry_point_worksheet.autofit();

    Ok(())
}

pub fn create_trust_boundary_worksheet(
    input_diagram: &InputDiagram,
    config: &Config,
    workbook: &mut Workbook,
) -> Result<(), ExcelError> {
    // Create some formats to use in the worksheet.
    let title_format = Format::new()
        .set_border(FormatBorder::Double)
        .set_align(FormatAlign::Center)
        .set_bold()
        .set_background_color(Color::Gray);
    let row_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_align(FormatAlign::Left)
        .set_background_color(Color::White);

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
            data_size = index;
        });

    // entry_point_worksheet.write_row(5, 1, column_titles);
    // Create a new table and configure it.
    let columns = vec![
        TableColumn::new().set_header("ID"),
        TableColumn::new().set_header("Description"),
        TableColumn::new().set_header("Limit of Access"),
        TableColumn::new().set_header("Level of Authorization"),
    ];
    entry_point_worksheet.write_row_matrix(1, 0, data).unwrap();
    let table = Table::new()
        .set_banded_rows(true)
        .set_style(TableStyle::Medium26)
        .set_columns(&columns)
        .set_total_row(true);

    entry_point_worksheet
        .add_table(0, 0, (data_size) as u32, (columns.len() - 1) as u16, &table)
        .unwrap();
    entry_point_worksheet.autofit();

    Ok(())
}
