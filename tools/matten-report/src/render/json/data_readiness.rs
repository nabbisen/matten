use std::error::Error;

use super::model::{
    JsonDataReadinessPayload, JsonMissingCount, JsonNumericConversion, json_tensor_preview,
    render_json_envelope,
};
use crate::report::data_readiness::{DataReadinessConversion, DataReadinessReportData};
use crate::request::KIND_DATA_READINESS;

pub(crate) mod input;

pub(crate) fn render(data: &DataReadinessReportData) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_DATA_READINESS, payload(data)?)
}

fn payload(data: &DataReadinessReportData) -> Result<JsonDataReadinessPayload, Box<dyn Error>> {
    let (tensor_shape, tensor_values) = match &data.conversion {
        DataReadinessConversion::Success {
            tensor_shape,
            tensor_values,
        } => (tensor_shape, tensor_values),
        DataReadinessConversion::Error { .. } => {
            return Err("fixed data-readiness JSON requires successful conversion".into());
        }
    };
    Ok(JsonDataReadinessPayload {
        input_label: data.input_label.clone(),
        source_columns: data.source_columns.clone(),
        selected_columns: data.selected_columns.clone(),
        left_out_columns: data.left_out_columns.clone(),
        missing_counts: data
            .missing_counts
            .iter()
            .map(|row| JsonMissingCount {
                column: row.column.clone(),
                missing: row.missing,
            })
            .collect(),
        numeric_conversion: JsonNumericConversion {
            status: "success",
            tensor: json_tensor_preview(tensor_shape, tensor_values)?,
        },
    })
}

#[cfg(test)]
mod tests;
