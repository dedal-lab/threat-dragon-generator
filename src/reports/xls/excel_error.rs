#[derive(Debug, Clone)]
pub enum ExcelError {
    SetColumnWidth(String),
    SetName(String),
    Write(String),
    WriteWithFormat(String),
    ExcelDateTime(String),
    MergeRange(String),
    NewImage(String),
    InsertImage(String),
    Save(String),
}
