use std::error::Error;

use super::model::{
    JsonConversionResult, JsonDynamicReadinessPayload, JsonDynamicValue,
    JsonExplicitPolicyConversion, JsonReadinessMasks, JsonSchemaSummaryRow, json_tensor_preview,
    render_json_envelope,
};
use crate::report::dynamic_readiness::DynamicReadinessReportData;
use crate::request::KIND_DYNAMIC_READINESS;

pub(crate) fn render(data: &DynamicReadinessReportData) -> Result<String, Box<dyn Error>> {
    render_json_envelope(KIND_DYNAMIC_READINESS, payload(data)?)
}

fn payload(
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

#[cfg(test)]
mod tests;
