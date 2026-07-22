use std::error::Error;

use serde::Serialize;

const MAX_TENSOR_PREVIEW_VALUES: usize = 12;

#[derive(Serialize)]
pub(crate) struct JsonReportEnvelope<T> {
    pub(crate) schema_version: u8,
    pub(crate) schema_status: &'static str,
    pub(crate) tool: &'static str,
    pub(crate) report_kind: &'static str,
    pub(crate) input_mode: &'static str,
    pub(crate) data: T,
}

#[derive(Serialize)]
pub(crate) struct JsonTensorPreview {
    pub(crate) shape: Vec<usize>,
    pub(crate) values: Vec<f64>,
    pub(crate) truncated: bool,
    pub(crate) shown_values: usize,
    pub(crate) total_values: usize,
    pub(crate) limit: usize,
}

#[derive(Serialize)]
pub(crate) struct JsonMissingCount {
    pub(crate) column: String,
    pub(crate) missing: usize,
}

#[derive(Serialize)]
pub(crate) struct JsonDataReadinessPayload {
    pub(crate) input_label: String,
    pub(crate) source_columns: Vec<String>,
    pub(crate) selected_columns: Vec<String>,
    pub(crate) left_out_columns: Vec<String>,
    pub(crate) missing_counts: Vec<JsonMissingCount>,
    pub(crate) numeric_conversion: JsonNumericConversion,
}

#[derive(Serialize)]
pub(crate) struct JsonNumericConversion {
    pub(crate) status: &'static str,
    pub(crate) tensor: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonShapeFlowPayload {
    pub(crate) broadcast: JsonBroadcastOperation,
    pub(crate) reshape: JsonReshapeOperation,
    pub(crate) axis_reductions: JsonAxisReductions,
    pub(crate) matmul: JsonMatmulOperation,
}

#[derive(Serialize)]
pub(crate) struct JsonBroadcastOperation {
    pub(crate) operation: &'static str,
    pub(crate) input_a_shape: Vec<usize>,
    pub(crate) input_b_shape: Vec<usize>,
    pub(crate) result: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonReshapeOperation {
    pub(crate) operation: &'static str,
    pub(crate) input_shape: Vec<usize>,
    pub(crate) result: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonAxisReductions {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) mean_axis_0: JsonTensorPreview,
    pub(crate) mean_axis_1: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonMatmulOperation {
    pub(crate) operation: &'static str,
    pub(crate) left_shape: Vec<usize>,
    pub(crate) right_shape: Vec<usize>,
    pub(crate) result: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonDynamicReadinessPayload {
    pub(crate) shape: Vec<usize>,
    pub(crate) values: Vec<JsonDynamicValue>,
    pub(crate) schema_summary: Vec<JsonSchemaSummaryRow>,
    pub(crate) readiness_masks: JsonReadinessMasks,
    pub(crate) strict_conversion: JsonConversionResult,
    pub(crate) explicit_policy_conversion: JsonExplicitPolicyConversion,
}

#[derive(Serialize)]
pub(crate) struct JsonDynamicValue {
    pub(crate) row: usize,
    pub(crate) column: usize,
    pub(crate) element: String,
}

#[derive(Serialize)]
pub(crate) struct JsonSchemaSummaryRow {
    pub(crate) label: &'static str,
    pub(crate) count: usize,
}

#[derive(Serialize)]
pub(crate) struct JsonReadinessMasks {
    pub(crate) none_mask: JsonTensorPreview,
    pub(crate) numeric_mask: JsonTensorPreview,
    pub(crate) strict_numeric_ready: bool,
}

#[derive(Serialize)]
pub(crate) struct JsonConversionResult {
    pub(crate) status: &'static str,
    pub(crate) message: &'static str,
}

#[derive(Serialize)]
pub(crate) struct JsonExplicitPolicyConversion {
    pub(crate) policy: &'static str,
    pub(crate) tensor: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonMlprepStandardizationPayload {
    pub(crate) selected_columns: Vec<&'static str>,
    pub(crate) operation: &'static str,
    pub(crate) before: JsonMlprepState,
    pub(crate) after: JsonMlprepState,
}

#[derive(Serialize)]
pub(crate) struct JsonMlprepState {
    pub(crate) tensor: JsonTensorPreview,
    pub(crate) column_mean: Vec<f64>,
    pub(crate) column_population_std: Vec<f64>,
}

#[derive(Serialize)]
pub(crate) struct JsonEducationalPathPayload {
    pub(crate) reading_steps: Vec<&'static str>,
    pub(crate) broadcasting: JsonEducationalBroadcast,
    pub(crate) reshape_and_transpose: JsonEducationalReshapeTranspose,
    pub(crate) axis_reductions: JsonEducationalAxisReductions,
    pub(crate) matmul: JsonEducationalMatmul,
    pub(crate) dynamic_readiness: JsonEducationalDynamicReadiness,
    pub(crate) standardization: JsonEducationalStandardization,
    pub(crate) non_goals: Vec<&'static str>,
}

#[derive(Serialize)]
pub(crate) struct JsonEducationalBroadcast {
    pub(crate) left_shape: Vec<usize>,
    pub(crate) right_shape: Vec<usize>,
    pub(crate) result: JsonTensorPreview,
    pub(crate) axis_1_meaning: &'static str,
    pub(crate) axis_0_meaning: &'static str,
}

#[derive(Serialize)]
pub(crate) struct JsonEducationalReshapeTranspose {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) reshape: JsonTensorPreview,
    pub(crate) transpose: JsonTensorPreview,
    pub(crate) meaning: &'static str,
}

#[derive(Serialize)]
pub(crate) struct JsonEducationalAxisReductions {
    pub(crate) input_shape: Vec<usize>,
    pub(crate) mean_axis_0: JsonTensorPreview,
    pub(crate) mean_axis_1: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonEducationalMatmul {
    pub(crate) left_shape: Vec<usize>,
    pub(crate) right_shape: Vec<usize>,
    pub(crate) shared_inner_dimension: usize,
    pub(crate) result: JsonTensorPreview,
}

#[derive(Serialize)]
pub(crate) struct JsonEducationalDynamicReadiness {
    pub(crate) shape: Vec<usize>,
    pub(crate) none_mask: JsonTensorPreview,
    pub(crate) numeric_mask: JsonTensorPreview,
    pub(crate) note: &'static str,
    pub(crate) next_step: &'static str,
}

#[derive(Serialize)]
pub(crate) struct JsonEducationalStandardization {
    pub(crate) operation: &'static str,
    pub(crate) input_shape: Vec<usize>,
    pub(crate) output_shape: Vec<usize>,
    pub(crate) before_mean: Vec<f64>,
    pub(crate) before_population_std: Vec<f64>,
    pub(crate) after_mean: Vec<f64>,
    pub(crate) after_population_std: Vec<f64>,
}

pub(crate) fn render_json_envelope<T: Serialize>(
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

pub(crate) fn json_tensor_preview(
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

pub(crate) fn ensure_finite_values(values: &[f64]) -> Result<(), Box<dyn Error>> {
    if values.iter().all(|value| value.is_finite()) {
        Ok(())
    } else {
        Err("JSON report encountered a non-finite numeric value".into())
    }
}

#[cfg(test)]
mod tests {
    use super::ensure_finite_values;

    #[test]
    fn non_finite_values_are_rejected() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let error = ensure_finite_values(&[value])
                .expect_err("non-finite JSON values must be rejected");
            assert_eq!(
                error.to_string(),
                "JSON report encountered a non-finite numeric value"
            );
        }
        ensure_finite_values(&[f64::MIN, 0.0, f64::MAX])
            .expect("finite JSON values should be accepted");
    }
}
