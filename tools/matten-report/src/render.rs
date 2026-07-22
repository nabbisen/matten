use std::error::Error;

use serde::Serialize;

use crate::report::data_readiness::{DataReadinessConversion, DataReadinessReportData};
use crate::report::dynamic_readiness::DynamicReadinessReportData;
use crate::report::educational_path::EducationalPathReportData;
use crate::report::mlprep_standardization::MlprepStandardizationReportData;
use crate::report::shape_flow::ShapeFlowReportData;
use crate::request::{
    KIND_DATA_READINESS, KIND_DYNAMIC_READINESS, KIND_EDUCATIONAL_PATH,
    KIND_MLPREP_STANDARDIZATION, KIND_SHAPE_FLOW,
};

mod common;
pub(crate) mod html;
pub(crate) mod markdown;

const MAX_TENSOR_PREVIEW_VALUES: usize = 12;

#[derive(Serialize)]
struct JsonReportEnvelope<T> {
    schema_version: u8,
    schema_status: &'static str,
    tool: &'static str,
    report_kind: &'static str,
    input_mode: &'static str,
    data: T,
}

#[derive(Serialize)]
struct JsonTensorPreview {
    shape: Vec<usize>,
    values: Vec<f64>,
    truncated: bool,
    shown_values: usize,
    total_values: usize,
    limit: usize,
}

#[derive(Serialize)]
struct JsonMissingCount {
    column: String,
    missing: usize,
}

#[derive(Serialize)]
struct JsonDataReadinessPayload {
    input_label: String,
    source_columns: Vec<String>,
    selected_columns: Vec<String>,
    left_out_columns: Vec<String>,
    missing_counts: Vec<JsonMissingCount>,
    numeric_conversion: JsonNumericConversion,
}

#[derive(Serialize)]
struct JsonNumericConversion {
    status: &'static str,
    tensor: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonShapeFlowPayload {
    broadcast: JsonBroadcastOperation,
    reshape: JsonReshapeOperation,
    axis_reductions: JsonAxisReductions,
    matmul: JsonMatmulOperation,
}

#[derive(Serialize)]
struct JsonBroadcastOperation {
    operation: &'static str,
    input_a_shape: Vec<usize>,
    input_b_shape: Vec<usize>,
    result: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonReshapeOperation {
    operation: &'static str,
    input_shape: Vec<usize>,
    result: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonAxisReductions {
    input_shape: Vec<usize>,
    mean_axis_0: JsonTensorPreview,
    mean_axis_1: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonMatmulOperation {
    operation: &'static str,
    left_shape: Vec<usize>,
    right_shape: Vec<usize>,
    result: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonDynamicReadinessPayload {
    shape: Vec<usize>,
    values: Vec<JsonDynamicValue>,
    schema_summary: Vec<JsonSchemaSummaryRow>,
    readiness_masks: JsonReadinessMasks,
    strict_conversion: JsonConversionResult,
    explicit_policy_conversion: JsonExplicitPolicyConversion,
}

#[derive(Serialize)]
struct JsonDynamicValue {
    row: usize,
    column: usize,
    element: String,
}

#[derive(Serialize)]
struct JsonSchemaSummaryRow {
    label: &'static str,
    count: usize,
}

#[derive(Serialize)]
struct JsonReadinessMasks {
    none_mask: JsonTensorPreview,
    numeric_mask: JsonTensorPreview,
    strict_numeric_ready: bool,
}

#[derive(Serialize)]
struct JsonConversionResult {
    status: &'static str,
    message: &'static str,
}

#[derive(Serialize)]
struct JsonExplicitPolicyConversion {
    policy: &'static str,
    tensor: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonMlprepStandardizationPayload {
    selected_columns: Vec<&'static str>,
    operation: &'static str,
    before: JsonMlprepState,
    after: JsonMlprepState,
}

#[derive(Serialize)]
struct JsonMlprepState {
    tensor: JsonTensorPreview,
    column_mean: Vec<f64>,
    column_population_std: Vec<f64>,
}

#[derive(Serialize)]
struct JsonEducationalPathPayload {
    reading_steps: Vec<&'static str>,
    broadcasting: JsonEducationalBroadcast,
    reshape_and_transpose: JsonEducationalReshapeTranspose,
    axis_reductions: JsonEducationalAxisReductions,
    matmul: JsonEducationalMatmul,
    dynamic_readiness: JsonEducationalDynamicReadiness,
    standardization: JsonEducationalStandardization,
    non_goals: Vec<&'static str>,
}

#[derive(Serialize)]
struct JsonEducationalBroadcast {
    left_shape: Vec<usize>,
    right_shape: Vec<usize>,
    result: JsonTensorPreview,
    axis_1_meaning: &'static str,
    axis_0_meaning: &'static str,
}

#[derive(Serialize)]
struct JsonEducationalReshapeTranspose {
    input_shape: Vec<usize>,
    reshape: JsonTensorPreview,
    transpose: JsonTensorPreview,
    meaning: &'static str,
}

#[derive(Serialize)]
struct JsonEducationalAxisReductions {
    input_shape: Vec<usize>,
    mean_axis_0: JsonTensorPreview,
    mean_axis_1: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonEducationalMatmul {
    left_shape: Vec<usize>,
    right_shape: Vec<usize>,
    shared_inner_dimension: usize,
    result: JsonTensorPreview,
}

#[derive(Serialize)]
struct JsonEducationalDynamicReadiness {
    shape: Vec<usize>,
    none_mask: JsonTensorPreview,
    numeric_mask: JsonTensorPreview,
    note: &'static str,
    next_step: &'static str,
}

#[derive(Serialize)]
struct JsonEducationalStandardization {
    operation: &'static str,
    input_shape: Vec<usize>,
    output_shape: Vec<usize>,
    before_mean: Vec<f64>,
    before_population_std: Vec<f64>,
    after_mean: Vec<f64>,
    after_population_std: Vec<f64>,
}

fn render_json_envelope<T: Serialize>(
    report_kind: &'static str,
    data: T,
) -> Result<String, Box<dyn Error>> {
    let envelope = JsonReportEnvelope {
        schema_version: 0,
        schema_status: "private-local",
        tool: "matten-report",
        report_kind,
        input_mode: "demo",
        data,
    };
    let mut report = serde_json::to_string_pretty(&envelope)?;
    report.push('\n');
    Ok(report)
}

fn json_tensor_preview(
    shape: &[usize],
    values: &[f64],
) -> Result<JsonTensorPreview, Box<dyn Error>> {
    ensure_finite_values(values)?;
    let shown_values = values.len().min(MAX_TENSOR_PREVIEW_VALUES);
    Ok(JsonTensorPreview {
        shape: shape.to_vec(),
        values: values.iter().copied().take(shown_values).collect(),
        truncated: values.len() > MAX_TENSOR_PREVIEW_VALUES,
        shown_values,
        total_values: values.len(),
        limit: MAX_TENSOR_PREVIEW_VALUES,
    })
}

fn ensure_finite_values(values: &[f64]) -> Result<(), Box<dyn Error>> {
    if values.iter().all(|value| value.is_finite()) {
        Ok(())
    } else {
        Err("JSON report encountered a non-finite numeric value".into())
    }
}

pub(crate) fn render_data_readiness_json_report(
    data: &DataReadinessReportData,
) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_DATA_READINESS, data_readiness_json_payload(data)?)
}

fn data_readiness_json_payload(
    data: &DataReadinessReportData,
) -> Result<JsonDataReadinessPayload, Box<dyn Error>> {
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

pub(crate) fn render_shape_flow_json_report(
    data: &ShapeFlowReportData,
) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_SHAPE_FLOW, shape_flow_json_payload(data)?)
}

fn shape_flow_json_payload(
    data: &ShapeFlowReportData,
) -> Result<JsonShapeFlowPayload, Box<dyn Error>> {
    Ok(JsonShapeFlowPayload {
        broadcast: JsonBroadcastOperation {
            operation: data.broadcast.operation,
            input_a_shape: data.broadcast.input_a_shape.clone(),
            input_b_shape: data.broadcast.input_b_shape.clone(),
            result: json_tensor_preview(
                &data.broadcast.result_shape,
                &data.broadcast.result_values,
            )?,
        },
        reshape: JsonReshapeOperation {
            operation: data.reshape.operation,
            input_shape: data.reshape.input_shape.clone(),
            result: json_tensor_preview(&data.reshape.result_shape, &data.reshape.result_values)?,
        },
        axis_reductions: JsonAxisReductions {
            input_shape: data.axis.input_shape.clone(),
            mean_axis_0: json_tensor_preview(
                &data.axis.mean_axis_0_shape,
                &data.axis.mean_axis_0_values,
            )?,
            mean_axis_1: json_tensor_preview(
                &data.axis.mean_axis_1_shape,
                &data.axis.mean_axis_1_values,
            )?,
        },
        matmul: JsonMatmulOperation {
            operation: data.matmul.operation,
            left_shape: data.matmul.left_shape.clone(),
            right_shape: data.matmul.right_shape.clone(),
            result: json_tensor_preview(&data.matmul.result_shape, &data.matmul.result_values)?,
        },
    })
}

pub(crate) fn render_dynamic_readiness_json_report(
    data: &DynamicReadinessReportData,
) -> Result<String, Box<dyn Error>> {
    render_json_envelope(
        KIND_DYNAMIC_READINESS,
        dynamic_readiness_json_payload(data)?,
    )
}

fn dynamic_readiness_json_payload(
    data: &DynamicReadinessReportData,
) -> Result<JsonDynamicReadinessPayload, Box<dyn Error>> {
    Ok(JsonDynamicReadinessPayload {
        shape: data.shape.clone(),
        values: data
            .values
            .iter()
            .map(|value| JsonDynamicValue {
                row: value.row,
                column: value.column,
                element: value.element.clone(),
            })
            .collect(),
        schema_summary: data
            .schema_summary
            .iter()
            .map(|row| JsonSchemaSummaryRow {
                label: row.label,
                count: row.count,
            })
            .collect(),
        readiness_masks: JsonReadinessMasks {
            none_mask: json_tensor_preview(&data.shape, &data.none_mask_values)?,
            numeric_mask: json_tensor_preview(&data.shape, &data.numeric_mask_values)?,
            strict_numeric_ready: data.strict_numeric_ready,
        },
        strict_conversion: JsonConversionResult {
            status: "error",
            message: data.strict_conversion_result,
        },
        explicit_policy_conversion: JsonExplicitPolicyConversion {
            policy: data.explicit_policy,
            tensor: json_tensor_preview(&data.converted_shape, &data.converted_values)?,
        },
    })
}

pub(crate) fn render_mlprep_standardization_json_report(
    data: &MlprepStandardizationReportData,
) -> Result<String, Box<dyn Error>> {
    render_json_envelope(
        KIND_MLPREP_STANDARDIZATION,
        mlprep_standardization_json_payload(data)?,
    )
}

fn mlprep_standardization_json_payload(
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

pub(crate) fn render_educational_path_json_report(
    data: &EducationalPathReportData,
) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_EDUCATIONAL_PATH, educational_path_json_payload(data)?)
}

fn educational_path_json_payload(
    data: &EducationalPathReportData,
) -> Result<JsonEducationalPathPayload, Box<dyn Error>> {
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
