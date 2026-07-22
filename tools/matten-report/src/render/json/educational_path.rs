use std::error::Error;

use super::model::{
    JsonEducationalAxisReductions, JsonEducationalBroadcast, JsonEducationalDynamicReadiness,
    JsonEducationalMatmul, JsonEducationalPathPayload, JsonEducationalReshapeTranspose,
    JsonEducationalStandardization, ensure_finite_values, json_tensor_preview,
    render_json_envelope,
};
use crate::report::educational_path::EducationalPathReportData;
use crate::request::KIND_EDUCATIONAL_PATH;

pub(crate) fn render(data: &EducationalPathReportData) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_EDUCATIONAL_PATH, payload(data)?)
}

fn payload(data: &EducationalPathReportData) -> Result<JsonEducationalPathPayload, Box<dyn Error>> {
    ensure_finite_values(&data.standardization.before_mean)?;
    ensure_finite_values(&data.standardization.before_std)?;
    ensure_finite_values(&data.standardization.after_mean)?;
    ensure_finite_values(&data.standardization.after_std)?;
    Ok(JsonEducationalPathPayload {
        reading_steps: data.reading_steps.to_vec(),
        broadcasting: JsonEducationalBroadcast {
            left_shape: data.broadcast.left_shape.clone(),
            right_shape: data.broadcast.right_shape.clone(),
            result: json_tensor_preview(
                &data.broadcast.result_shape,
                &data.broadcast.result_values,
            )?,
            axis_1_meaning: "left repeats across 4 columns",
            axis_0_meaning: "right repeats across 3 rows",
        },
        reshape_and_transpose: JsonEducationalReshapeTranspose {
            input_shape: data.reshape_transpose.input_shape.clone(),
            reshape: json_tensor_preview(
                &data.reshape_transpose.reshape_shape,
                &data.reshape_transpose.reshape_values,
            )?,
            transpose: json_tensor_preview(
                &data.reshape_transpose.transpose_shape,
                &data.reshape_transpose.transpose_values,
            )?,
            meaning: "reshape changes grouping; transpose changes coordinate meaning",
        },
        axis_reductions: JsonEducationalAxisReductions {
            input_shape: data.axis_reductions.input_shape.clone(),
            mean_axis_0: json_tensor_preview(
                &data.axis_reductions.mean_axis_0_shape,
                &data.axis_reductions.mean_axis_0_values,
            )?,
            mean_axis_1: json_tensor_preview(
                &data.axis_reductions.mean_axis_1_shape,
                &data.axis_reductions.mean_axis_1_values,
            )?,
        },
        matmul: JsonEducationalMatmul {
            left_shape: data.matmul.left_shape.clone(),
            right_shape: data.matmul.right_shape.clone(),
            shared_inner_dimension: data.matmul.shared_inner_dimension,
            result: json_tensor_preview(&data.matmul.result_shape, &data.matmul.result_values)?,
        },
        dynamic_readiness: JsonEducationalDynamicReadiness {
            shape: data.dynamic_readiness.shape.clone(),
            none_mask: json_tensor_preview(
                &data.dynamic_readiness.shape,
                &data.dynamic_readiness.none_mask_values,
            )?,
            numeric_mask: json_tensor_preview(
                &data.dynamic_readiness.shape,
                &data.dynamic_readiness.numeric_mask_values,
            )?,
            note: "Text values are not numeric-ready under the strict mask",
            next_step: "clean values, then call try_numeric()",
        },
        standardization: JsonEducationalStandardization {
            operation: "standardize_columns(input)",
            input_shape: data.standardization.input_shape.clone(),
            output_shape: data.standardization.output_shape.clone(),
            before_mean: data.standardization.before_mean.clone(),
            before_population_std: data.standardization.before_std.clone(),
            after_mean: data.standardization.after_mean.clone(),
            after_population_std: data.standardization.after_std.clone(),
        },
        non_goals: data.non_goals.to_vec(),
    })
}

#[cfg(test)]
mod tests;
