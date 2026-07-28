use std::error::Error;

use serde::Serialize;

use super::super::model::{JsonTensorPreview, json_tensor_preview};
use crate::render::common::{
    MAX_DISPLAY_CHARS, MAX_DISPLAY_COLUMNS, MAX_ERROR_CHARS, MAX_TENSOR_PREVIEW_VALUES,
};
use crate::report::data_readiness::{
    DataReadinessConversion, DataReadinessMissingCount, DataReadinessReportData,
};
use crate::request::KIND_DATA_READINESS;

#[derive(Serialize)]
struct JsonInputReportEnvelope<T> {
    schema_version: u8,
    schema_status: &'static str,
    tool: &'static str,
    report_kind: &'static str,
    input_mode: &'static str,
    limits: JsonInputLimits,
    data: T,
}

#[derive(Serialize)]
struct JsonInputLimits {
    max_display_columns: usize,
    max_display_chars: usize,
    max_error_chars: usize,
    max_tensor_preview_values: usize,
}

#[derive(Serialize)]
struct JsonBoundedString {
    value: String,
    truncated: bool,
    shown_chars: usize,
    total_chars: usize,
    limit: usize,
}

#[derive(Serialize)]
struct JsonBoundedList<T> {
    items: Vec<T>,
    truncated: bool,
    shown_items: usize,
    total_items: usize,
    limit: usize,
}

#[derive(Serialize)]
struct JsonInputMissingCount {
    column: JsonBoundedString,
    missing: usize,
}

#[derive(Serialize)]
struct JsonInputDataReadinessPayload {
    input_label: JsonBoundedString,
    source_columns: JsonBoundedList<JsonBoundedString>,
    selected_columns: JsonBoundedList<JsonBoundedString>,
    left_out_columns: JsonBoundedList<JsonBoundedString>,
    missing_counts: JsonBoundedList<JsonInputMissingCount>,
    numeric_conversion: JsonInputNumericConversion,
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
enum JsonInputNumericConversion {
    Success { tensor: JsonTensorPreview },
    Error { message: JsonBoundedString },
}

pub(crate) fn render(data: &DataReadinessReportData) -> Result<String, Box<dyn Error>> {
    let envelope = JsonInputReportEnvelope {
        schema_version: 0,
        schema_status: "private-local",
        tool: "matten-report",
        report_kind: KIND_DATA_READINESS,
        input_mode: "csv",
        limits: JsonInputLimits {
            max_display_columns: MAX_DISPLAY_COLUMNS,
            max_display_chars: MAX_DISPLAY_CHARS,
            max_error_chars: MAX_ERROR_CHARS,
            max_tensor_preview_values: MAX_TENSOR_PREVIEW_VALUES,
        },
        data: payload(data)?,
    };
    let mut report = serde_json::to_string_pretty(&envelope)?;
    report.push('\n');
    Ok(report)
}

fn payload(
    data: &DataReadinessReportData,
) -> Result<JsonInputDataReadinessPayload, Box<dyn Error>> {
    let numeric_conversion = match &data.conversion {
        DataReadinessConversion::Success {
            tensor_shape,
            tensor_values,
        } => JsonInputNumericConversion::Success {
            tensor: json_tensor_preview(tensor_shape, tensor_values)?,
        },
        DataReadinessConversion::Error { message } => JsonInputNumericConversion::Error {
            message: bounded_string(message, MAX_ERROR_CHARS),
        },
    };

    Ok(JsonInputDataReadinessPayload {
        input_label: bounded_string(&data.input_label, MAX_DISPLAY_CHARS),
        source_columns: bounded_strings(&data.source_columns),
        selected_columns: bounded_strings(&data.selected_columns),
        left_out_columns: bounded_strings(&data.left_out_columns),
        missing_counts: bounded_missing_counts(&data.missing_counts),
        numeric_conversion,
    })
}

fn bounded_string(value: &str, limit: usize) -> JsonBoundedString {
    let total_chars = value.chars().count();
    let value: String = value.chars().take(limit).collect();
    let shown_chars = value.chars().count();
    JsonBoundedString {
        value,
        truncated: total_chars > limit,
        shown_chars,
        total_chars,
        limit,
    }
}

fn bounded_strings(values: &[String]) -> JsonBoundedList<JsonBoundedString> {
    bounded_list(
        values
            .iter()
            .take(MAX_DISPLAY_COLUMNS)
            .map(|value| bounded_string(value, MAX_DISPLAY_CHARS))
            .collect(),
        values.len(),
        MAX_DISPLAY_COLUMNS,
    )
}

fn bounded_missing_counts(
    values: &[DataReadinessMissingCount],
) -> JsonBoundedList<JsonInputMissingCount> {
    bounded_list(
        values
            .iter()
            .take(MAX_DISPLAY_COLUMNS)
            .map(|row| JsonInputMissingCount {
                column: bounded_string(&row.column, MAX_DISPLAY_CHARS),
                missing: row.missing,
            })
            .collect(),
        values.len(),
        MAX_DISPLAY_COLUMNS,
    )
}

fn bounded_list<T>(items: Vec<T>, total_items: usize, limit: usize) -> JsonBoundedList<T> {
    let shown_items = items.len();
    JsonBoundedList {
        items,
        truncated: total_items > limit,
        shown_items,
        total_items,
        limit,
    }
}

#[cfg(test)]
mod tests;
