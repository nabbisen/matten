use std::error::Error;

use matten_data::{MattenDataError, Table};

const DEMO_CSV: &str = "\
region,sales,cost,note
north,100,40,ok
south,150,45,review
east,120,55,ok";

pub(crate) struct DataReadinessReportData {
    pub(crate) input_label: String,
    pub(crate) source_columns: Vec<String>,
    pub(crate) selected_columns: Vec<String>,
    pub(crate) left_out_columns: Vec<String>,
    pub(crate) missing_counts: Vec<DataReadinessMissingCount>,
    pub(crate) conversion: DataReadinessConversion,
}

pub(crate) struct DataReadinessMissingCount {
    pub(crate) column: String,
    pub(crate) missing: usize,
}

pub(crate) enum DataReadinessConversion {
    Success {
        tensor_shape: Vec<usize>,
        tensor_values: Vec<f64>,
    },
    Error {
        message: String,
    },
}

pub(crate) fn build_demo() -> Result<DataReadinessReportData, Box<dyn Error>> {
    let table = Table::from_csv_str(DEMO_CSV).map_err(Box::<dyn Error>::from)?;
    build(
        "demo: data-readiness",
        &table,
        &["sales".to_string(), "cost".to_string()],
    )
}

pub(crate) fn build(
    input_label: &str,
    table: &Table,
    select: &[String],
) -> Result<DataReadinessReportData, Box<dyn Error>> {
    let selected = table
        .select_columns(select.iter().map(String::as_str))
        .map_err(Box::<dyn Error>::from)?;
    let selected_summary = selected.schema_summary();
    let missing_counts = selected_summary
        .column_summaries()
        .iter()
        .map(|column| DataReadinessMissingCount {
            column: column.name.clone(),
            missing: column.missing,
        })
        .collect();
    let conversion = match selected.try_numeric() {
        Ok(numeric) => {
            let tensor = numeric.to_tensor().map_err(Box::<dyn Error>::from)?;
            DataReadinessConversion::Success {
                tensor_shape: tensor.shape().to_vec(),
                tensor_values: tensor.as_slice().to_vec(),
            }
        }
        Err(err) => DataReadinessConversion::Error {
            message: describe_data_error(&err),
        },
    };

    Ok(DataReadinessReportData {
        input_label: input_label.to_string(),
        source_columns: table.column_names().to_vec(),
        selected_columns: select.to_vec(),
        left_out_columns: left_out_columns(table.column_names(), select),
        missing_counts,
        conversion,
    })
}

fn left_out_columns(source: &[String], selected: &[String]) -> Vec<String> {
    source
        .iter()
        .filter(|name| !selected.iter().any(|selected| selected == *name))
        .cloned()
        .collect()
}

fn describe_data_error(err: &MattenDataError) -> String {
    match err {
        MattenDataError::MissingValue { column, row } => {
            format!("missing value in column {column:?}, CSV line {row}")
        }
        MattenDataError::NonNumericValue { column, row, value } => {
            format!("non-numeric value {value:?} in column {column:?}, CSV line {row}")
        }
        MattenDataError::MissingColumn { name } => {
            format!("selected column {name:?} does not exist")
        }
        MattenDataError::DuplicateSelection { name } => {
            format!("selected column {name:?} was requested more than once")
        }
        MattenDataError::EmptySelection => "no columns were selected".to_string(),
        other => other.to_string(),
    }
}
