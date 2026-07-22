use std::error::Error;

use super::model::{
    JsonMlprepStandardizationPayload, JsonMlprepState, ensure_finite_values, json_tensor_preview,
    render_json_envelope,
};
use crate::report::mlprep_standardization::MlprepStandardizationReportData;
use crate::request::KIND_MLPREP_STANDARDIZATION;

pub(crate) fn render(data: &MlprepStandardizationReportData) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_MLPREP_STANDARDIZATION, payload(data)?)
}

fn payload(
    data: &MlprepStandardizationReportData,
) -> Result<JsonMlprepStandardizationPayload, Box<dyn Error>> {
    ensure_finite_values(&data.before_mean)?;
    ensure_finite_values(&data.before_std)?;
    ensure_finite_values(&data.after_mean)?;
    ensure_finite_values(&data.after_std)?;
    Ok(JsonMlprepStandardizationPayload {
        selected_columns: vec!["feature_0", "feature_1"],
        operation: "standardize_columns(input)",
        before: JsonMlprepState {
            tensor: json_tensor_preview(&data.input_shape, &data.input_values)?,
            column_mean: data.before_mean.clone(),
            column_population_std: data.before_std.clone(),
        },
        after: JsonMlprepState {
            tensor: json_tensor_preview(&data.output_shape, &data.output_values)?,
            column_mean: data.after_mean.clone(),
            column_population_std: data.after_std.clone(),
        },
    })
}

#[cfg(test)]
mod tests;
