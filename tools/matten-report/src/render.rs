use std::error::Error;
use std::fmt::Write as _;

use matten::{Element, Tensor};
use matten_data::{MattenDataError, Table};
use matten_mlprep::standardize_columns;
use serde::Serialize;

use crate::report::dynamic_readiness::DynamicReadinessReportData;
use crate::report::shape_flow::ShapeFlowReportData;
use crate::request::{
    KIND_DATA_READINESS, KIND_DYNAMIC_READINESS, KIND_EDUCATIONAL_PATH,
    KIND_MLPREP_STANDARDIZATION, KIND_SHAPE_FLOW, SUPPORTED_DEMOS,
};

const DEMO_CSV: &str = "\
region,sales,cost,note
north,100,40,ok
south,150,45,review
east,120,55,ok";
const MAX_DISPLAY_COLUMNS: usize = 12;
const MAX_DISPLAY_CHARS: usize = 120;
const MAX_ERROR_CHARS: usize = 240;
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
    input_label: &'static str,
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

pub(crate) fn render_fixed_demo_json_report(label: &str) -> Result<String, Box<dyn Error>> {
    match label {
        KIND_DATA_READINESS => {
            render_json_envelope(KIND_DATA_READINESS, data_readiness_json_payload()?)
        }
        KIND_SHAPE_FLOW => Err("shape-flow JSON requires prebuilt report data".into()),
        KIND_DYNAMIC_READINESS => {
            Err("dynamic-readiness JSON requires prebuilt report data".into())
        }
        KIND_MLPREP_STANDARDIZATION => render_json_envelope(
            KIND_MLPREP_STANDARDIZATION,
            mlprep_standardization_json_payload()?,
        ),
        KIND_EDUCATIONAL_PATH => {
            render_json_envelope(KIND_EDUCATIONAL_PATH, educational_path_json_payload()?)
        }
        other => Err(format!(
            "--format json is only supported for --demo {}; got {other:?}",
            SUPPORTED_DEMOS
        )
        .into()),
    }
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

fn data_readiness_json_payload() -> Result<JsonDataReadinessPayload, Box<dyn Error>> {
    let data = data_readiness_demo_report_data()?;
    Ok(JsonDataReadinessPayload {
        input_label: data.input_label,
        source_columns: data.source_columns,
        selected_columns: data.selected_columns,
        left_out_columns: data.left_out_columns,
        missing_counts: data
            .missing_counts
            .into_iter()
            .map(|row| JsonMissingCount {
                column: row.column,
                missing: row.missing,
            })
            .collect(),
        numeric_conversion: JsonNumericConversion {
            status: data.conversion_status,
            tensor: json_tensor_preview(&data.tensor_shape, &data.tensor_values)?,
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

fn mlprep_standardization_json_payload() -> Result<JsonMlprepStandardizationPayload, Box<dyn Error>>
{
    let data = mlprep_standardization_report_data()?;
    ensure_finite_values(&data.before_mean)?;
    ensure_finite_values(&data.before_std)?;
    ensure_finite_values(&data.after_mean)?;
    ensure_finite_values(&data.after_std)?;
    Ok(JsonMlprepStandardizationPayload {
        selected_columns: vec!["feature_0", "feature_1"],
        operation: "standardize_columns(input)",
        before: JsonMlprepState {
            tensor: json_tensor_preview(&data.input_shape, &data.input_values)?,
            column_mean: data.before_mean,
            column_population_std: data.before_std,
        },
        after: JsonMlprepState {
            tensor: json_tensor_preview(&data.output_shape, &data.output_values)?,
            column_mean: data.after_mean,
            column_population_std: data.after_std,
        },
    })
}

fn educational_path_json_payload() -> Result<JsonEducationalPathPayload, Box<dyn Error>> {
    let data = educational_path_report_data()?;
    ensure_finite_values(&data.standardization.before_mean)?;
    ensure_finite_values(&data.standardization.before_std)?;
    ensure_finite_values(&data.standardization.after_mean)?;
    ensure_finite_values(&data.standardization.after_std)?;
    Ok(JsonEducationalPathPayload {
        reading_steps: data.reading_steps.to_vec(),
        broadcasting: JsonEducationalBroadcast {
            left_shape: data.broadcast.left_shape,
            right_shape: data.broadcast.right_shape,
            result: json_tensor_preview(
                &data.broadcast.result_shape,
                &data.broadcast.result_values,
            )?,
            axis_1_meaning: "left repeats across 4 columns",
            axis_0_meaning: "right repeats across 3 rows",
        },
        reshape_and_transpose: JsonEducationalReshapeTranspose {
            input_shape: data.reshape_transpose.input_shape,
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
            input_shape: data.axis_reductions.input_shape,
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
            left_shape: data.matmul.left_shape,
            right_shape: data.matmul.right_shape,
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
            input_shape: data.standardization.input_shape,
            output_shape: data.standardization.output_shape,
            before_mean: data.standardization.before_mean,
            before_population_std: data.standardization.before_std,
            after_mean: data.standardization.after_mean,
            after_population_std: data.standardization.after_std,
        },
        non_goals: data.non_goals.to_vec(),
    })
}

pub(crate) fn render_table_report(
    input_label: &str,
    table: &Table,
    select: &[String],
) -> Result<String, Box<dyn Error>> {
    let selected = table
        .select_columns(select.iter().map(String::as_str))
        .map_err(Box::<dyn Error>::from)?;
    let left_out = left_out_columns(table.column_names(), select);
    let selected_summary = selected.schema_summary();

    let mut report = String::new();
    writeln!(report, "# matten data-readiness report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "{input_label}")?;
    writeln!(report)?;

    writeln!(report, "## Source columns")?;
    write_list(&mut report, table.column_names())?;
    writeln!(report)?;

    writeln!(report, "## Selected columns")?;
    write_list(&mut report, select)?;
    writeln!(report)?;

    writeln!(report, "## Columns left out")?;
    write_list(&mut report, &left_out)?;
    writeln!(report)?;

    writeln!(report, "## Missing values")?;
    writeln!(report, "| column | missing |")?;
    writeln!(report, "|---|---:|")?;
    for column in selected_summary.column_summaries() {
        writeln!(report, "| {} | {} |", column.name, column.missing)?;
    }
    writeln!(report)?;

    writeln!(report, "## Numeric conversion")?;
    match selected.try_numeric() {
        Ok(numeric) => {
            writeln!(report, "strict conversion: success")?;
            writeln!(report)?;
            let tensor = numeric.to_tensor().map_err(Box::<dyn Error>::from)?;
            writeln!(report, "## Tensor preview")?;
            writeln!(report, "shape: {:?}", tensor.shape())?;
            writeln!(report, "row-major values: {:?}", tensor.as_slice())?;
        }
        Err(err) => {
            writeln!(
                report,
                "strict conversion: error: {}",
                describe_data_error(&err)
            )?;
        }
    }

    Ok(report)
}

pub(crate) fn render_data_readiness_markdown_report(
    select: &[String],
) -> Result<String, Box<dyn Error>> {
    let table = Table::from_csv_str(DEMO_CSV).map_err(Box::<dyn Error>::from)?;
    render_table_report("demo: data-readiness", &table, select)
}

struct DataReadinessReportData {
    input_label: &'static str,
    source_columns: Vec<String>,
    selected_columns: Vec<String>,
    left_out_columns: Vec<String>,
    missing_counts: Vec<DataReadinessMissingCount>,
    conversion_status: &'static str,
    tensor_shape: Vec<usize>,
    tensor_values: Vec<f64>,
}

struct DataReadinessMissingCount {
    column: String,
    missing: usize,
}

fn data_readiness_demo_report_data() -> Result<DataReadinessReportData, Box<dyn Error>> {
    let table = Table::from_csv_str(DEMO_CSV).map_err(Box::<dyn Error>::from)?;
    let selected_columns = vec!["sales".to_string(), "cost".to_string()];
    let selected = table
        .select_columns(selected_columns.iter().map(String::as_str))
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
    let numeric = selected.try_numeric().map_err(Box::<dyn Error>::from)?;
    let tensor = numeric.to_tensor().map_err(Box::<dyn Error>::from)?;

    Ok(DataReadinessReportData {
        input_label: "demo: data-readiness",
        source_columns: table.column_names().to_vec(),
        left_out_columns: left_out_columns(table.column_names(), &selected_columns),
        selected_columns,
        missing_counts,
        conversion_status: "success",
        tensor_shape: tensor.shape().to_vec(),
        tensor_values: tensor.as_slice().to_vec(),
    })
}

pub(crate) fn render_data_readiness_html_report() -> Result<String, Box<dyn Error>> {
    let data = data_readiness_demo_report_data()?;
    render_html_document(
        "matten data-readiness report",
        "Fixed demo report, not arbitrary CSV profiling.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Input"))?;
            write_shape_flow_table(report, &[("input", data.input_label.to_string())])?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Columns"))?;
            write_shape_flow_table(
                report,
                &[
                    ("source columns", data.source_columns.join(", ")),
                    ("selected columns", data.selected_columns.join(", ")),
                    ("columns left out", data.left_out_columns.join(", ")),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Missing values"))?;
            writeln!(report, "<table>")?;
            writeln!(
                report,
                "<thead><tr><th>{}</th><th>{}</th></tr></thead>",
                html_escape("column"),
                html_escape("missing")
            )?;
            writeln!(report, "<tbody>")?;
            for row in &data.missing_counts {
                writeln!(
                    report,
                    "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
                    html_escape(&row.column),
                    row.missing
                )?;
            }
            writeln!(report, "</tbody>")?;
            writeln!(report, "</table>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Numeric conversion"))?;
            write_shape_flow_table(
                report,
                &[("strict conversion", data.conversion_status.to_string())],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Tensor preview"))?;
            write_shape_flow_table(
                report,
                &[
                    ("shape", format!("{:?}", data.tensor_shape)),
                    ("row-major values", format!("{:?}", data.tensor_values)),
                ],
            )?;
            writeln!(report, "</section>")
        },
    )
}

struct InputDataReadinessReportData {
    input_label: String,
    source_columns: Vec<String>,
    selected_columns: Vec<String>,
    left_out_columns: Vec<String>,
    missing_counts: Vec<DataReadinessMissingCount>,
    conversion: InputDataReadinessConversion,
}

enum InputDataReadinessConversion {
    Success {
        tensor_shape: Vec<usize>,
        tensor_values: Vec<f64>,
    },
    Error {
        message: String,
    },
}

fn input_data_readiness_report_data(
    input_label: &str,
    table: &Table,
    select: &[String],
) -> Result<InputDataReadinessReportData, Box<dyn Error>> {
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
            InputDataReadinessConversion::Success {
                tensor_shape: tensor.shape().to_vec(),
                tensor_values: tensor.as_slice().to_vec(),
            }
        }
        Err(err) => InputDataReadinessConversion::Error {
            message: describe_data_error(&err),
        },
    };

    Ok(InputDataReadinessReportData {
        input_label: input_label.to_string(),
        source_columns: table.column_names().to_vec(),
        selected_columns: select.to_vec(),
        left_out_columns: left_out_columns(table.column_names(), select),
        missing_counts,
        conversion,
    })
}

pub(crate) fn render_input_data_readiness_html_report(
    input_label: &str,
    table: &Table,
    select: &[String],
) -> Result<String, Box<dyn Error>> {
    let data = input_data_readiness_report_data(input_label, table, select)?;
    render_html_document(
        "matten data-readiness report",
        "Bounded summary of the provided CSV file; not a full raw table rendering.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Input"))?;
            write_shape_flow_table(
                report,
                &[("input", cap_display(&data.input_label, MAX_DISPLAY_CHARS))],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Columns"))?;
            write_shape_flow_table(
                report,
                &[
                    ("source columns", format_display_list(&data.source_columns)),
                    (
                        "selected columns",
                        format_display_list(&data.selected_columns),
                    ),
                    (
                        "columns left out",
                        format_display_list(&data.left_out_columns),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Missing values"))?;
            writeln!(report, "<table>")?;
            writeln!(
                report,
                "<thead><tr><th>{}</th><th>{}</th></tr></thead>",
                html_escape("column"),
                html_escape("missing")
            )?;
            writeln!(report, "<tbody>")?;
            for row in data.missing_counts.iter().take(MAX_DISPLAY_COLUMNS) {
                writeln!(
                    report,
                    "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
                    html_escape(&cap_display(&row.column, MAX_DISPLAY_CHARS)),
                    row.missing
                )?;
            }
            if data.missing_counts.len() > MAX_DISPLAY_COLUMNS {
                writeln!(
                    report,
                    "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
                    html_escape(&format!(
                        "... {} more",
                        data.missing_counts.len() - MAX_DISPLAY_COLUMNS
                    )),
                    html_escape("not shown")
                )?;
            }
            writeln!(report, "</tbody>")?;
            writeln!(report, "</table>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Numeric conversion"))?;
            match &data.conversion {
                InputDataReadinessConversion::Success {
                    tensor_shape,
                    tensor_values,
                } => {
                    write_shape_flow_table(
                        report,
                        &[("strict conversion", "success".to_string())],
                    )?;
                    writeln!(report, "</section>")?;

                    writeln!(report, "<section>")?;
                    writeln!(report, "<h2>{}</h2>", html_escape("Tensor preview"))?;
                    write_shape_flow_table(
                        report,
                        &[
                            ("shape", format!("{tensor_shape:?}")),
                            ("row-major values", format_tensor_preview(tensor_values)),
                        ],
                    )?;
                }
                InputDataReadinessConversion::Error { message } => {
                    write_shape_flow_table(
                        report,
                        &[
                            ("strict conversion", "error".to_string()),
                            ("error", cap_display(message, MAX_ERROR_CHARS)),
                        ],
                    )?;
                }
            }
            writeln!(report, "</section>")
        },
    )
}

fn cap_display(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let keep = max_chars.saturating_sub(3);
    let mut capped: String = value.chars().take(keep).collect();
    capped.push_str("...");
    capped
}

fn format_display_list(values: &[String]) -> String {
    let mut parts: Vec<String> = values
        .iter()
        .take(MAX_DISPLAY_COLUMNS)
        .map(|value| cap_display(value, MAX_DISPLAY_CHARS))
        .collect();
    if values.len() > MAX_DISPLAY_COLUMNS {
        parts.push(format!("... {} more", values.len() - MAX_DISPLAY_COLUMNS));
    }
    parts.join(", ")
}

fn format_tensor_preview(values: &[f64]) -> String {
    let mut parts: Vec<String> = values
        .iter()
        .take(MAX_TENSOR_PREVIEW_VALUES)
        .map(|value| format!("{value:?}"))
        .collect();
    if values.len() > MAX_TENSOR_PREVIEW_VALUES {
        parts.push(format!(
            "... {} more",
            values.len() - MAX_TENSOR_PREVIEW_VALUES
        ));
    }
    format!("[{}]", parts.join(", "))
}

pub(crate) fn render_shape_flow_report(
    data: &ShapeFlowReportData,
) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    writeln!(report, "# matten shape-flow report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_SHAPE_FLOW}")?;
    writeln!(
        report,
        "note: fixed demo report, not automatic expression tracing"
    )?;
    writeln!(report)?;

    writeln!(report, "## Broadcast add")?;
    writeln!(report, "input a: shape {:?}", data.broadcast.input_a_shape)?;
    writeln!(report, "input b: shape {:?}", data.broadcast.input_b_shape)?;
    writeln!(report, "operation: {}", data.broadcast.operation)?;
    writeln!(
        report,
        "shape flow: {:?} + {:?} -> {:?}",
        data.broadcast.input_a_shape, data.broadcast.input_b_shape, data.broadcast.result_shape
    )?;
    writeln!(report, "result values: {:?}", data.broadcast.result_values)?;
    writeln!(report)?;

    writeln!(report, "## Reshape")?;
    writeln!(report, "input: shape {:?}", data.reshape.input_shape)?;
    writeln!(report, "operation: {}", data.reshape.operation)?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        data.reshape.input_shape, data.reshape.result_shape
    )?;
    writeln!(report, "result values: {:?}", data.reshape.result_values)?;
    writeln!(report)?;

    writeln!(report, "## Axis reductions")?;
    writeln!(report, "input: shape {:?}", data.axis.input_shape)?;
    writeln!(
        report,
        "mean_axis(0): {:?} -> {:?}",
        data.axis.input_shape, data.axis.mean_axis_0_shape
    )?;
    writeln!(
        report,
        "mean_axis(0) values: {:?}",
        data.axis.mean_axis_0_values
    )?;
    writeln!(
        report,
        "mean_axis(1): {:?} -> {:?}",
        data.axis.input_shape, data.axis.mean_axis_1_shape
    )?;
    writeln!(
        report,
        "mean_axis(1) values: {:?}",
        data.axis.mean_axis_1_values
    )?;
    writeln!(report)?;

    writeln!(report, "## Matrix multiplication")?;
    writeln!(report, "left: shape {:?}", data.matmul.left_shape)?;
    writeln!(report, "right: shape {:?}", data.matmul.right_shape)?;
    writeln!(report, "operation: {}", data.matmul.operation)?;
    writeln!(
        report,
        "shape flow: {:?} @ {:?} -> {:?}",
        data.matmul.left_shape, data.matmul.right_shape, data.matmul.result_shape
    )?;
    writeln!(report, "result values: {:?}", data.matmul.result_values)?;

    Ok(report)
}

pub(crate) fn render_shape_flow_html_report(
    data: &ShapeFlowReportData,
) -> Result<String, Box<dyn Error>> {
    render_html_document(
        "matten shape-flow report",
        "Fixed demo report, not automatic expression tracing.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Broadcast add"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input a", format!("{:?}", data.broadcast.input_a_shape)),
                    ("input b", format!("{:?}", data.broadcast.input_b_shape)),
                    ("result", format!("{:?}", data.broadcast.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                html_escape(&format!("operation: {}", data.broadcast.operation))
            )?;
            write_html_pre(
                report,
                &format!("result values: {:?}", data.broadcast.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Reshape"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input", format!("{:?}", data.reshape.input_shape)),
                    ("result", format!("{:?}", data.reshape.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                html_escape(&format!("operation: {}", data.reshape.operation))
            )?;
            write_html_pre(
                report,
                &format!("result values: {:?}", data.reshape.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Axis reductions"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input", format!("{:?}", data.axis.input_shape)),
                    (
                        "mean_axis(0)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis.input_shape, data.axis.mean_axis_0_shape
                        ),
                    ),
                    (
                        "mean_axis(1)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis.input_shape, data.axis.mean_axis_1_shape
                        ),
                    ),
                ],
            )?;
            write_html_pre(
                report,
                &format!(
                    "mean_axis(0) values: {:?}\nmean_axis(1) values: {:?}",
                    data.axis.mean_axis_0_values, data.axis.mean_axis_1_values
                ),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Matrix multiplication"))?;
            write_shape_flow_table(
                report,
                &[
                    ("left", format!("{:?}", data.matmul.left_shape)),
                    ("right", format!("{:?}", data.matmul.right_shape)),
                    ("result", format!("{:?}", data.matmul.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                html_escape(&format!("operation: {}", data.matmul.operation))
            )?;
            write_html_pre(
                report,
                &format!("result values: {:?}", data.matmul.result_values),
            )?;
            writeln!(report, "</section>")
        },
    )
}

pub(crate) fn render_dynamic_readiness_report(
    data: &DynamicReadinessReportData,
) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    writeln!(report, "# matten dynamic-readiness report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_DYNAMIC_READINESS}")?;
    writeln!(
        report,
        "note: fixed demo report, not automatic data profiling"
    )?;
    writeln!(report)?;

    writeln!(report, "## Dynamic values")?;
    writeln!(report, "shape: {:?}", data.shape)?;
    writeln!(report, "row-major values:")?;
    for value in &data.values {
        writeln!(
            report,
            "- [{}, {}] {}",
            value.row, value.column, value.element
        )?;
    }
    writeln!(report, "schema summary:")?;
    for row in &data.schema_summary {
        writeln!(report, "- {}: {}", row.label, row.count)?;
    }
    writeln!(report)?;

    writeln!(report, "## Readiness masks")?;
    writeln!(report, "none mask: {:?}", data.none_mask_values)?;
    writeln!(
        report,
        "numeric mask: strict policy readiness {:?}",
        data.numeric_mask_values
    )?;
    writeln!(
        report,
        "strict numeric-ready: {}",
        data.strict_numeric_ready
    )?;
    writeln!(report)?;

    writeln!(report, "## Strict conversion")?;
    writeln!(report, "result: {}", data.strict_conversion_result)?;
    writeln!(report)?;

    writeln!(report, "## Explicit policy conversion")?;
    writeln!(report, "policy: {}", data.explicit_policy)?;
    writeln!(report, "converted shape: {:?}", data.converted_shape)?;
    writeln!(
        report,
        "converted row-major values: {:?}",
        data.converted_values
    )?;

    Ok(report)
}

pub(crate) fn render_dynamic_readiness_html_report(
    data: &DynamicReadinessReportData,
) -> Result<String, Box<dyn Error>> {
    render_html_document(
        "matten dynamic-readiness report",
        "Fixed demo report, not automatic data profiling.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Dynamic values"))?;
            write_shape_flow_table(report, &[("shape", format!("{:?}", data.shape))])?;
            writeln!(report, "<table>")?;
            writeln!(
                report,
                "<thead><tr><th>{}</th><th>{}</th><th>{}</th></tr></thead>",
                html_escape("row"),
                html_escape("column"),
                html_escape("value")
            )?;
            writeln!(report, "<tbody>")?;
            for value in &data.values {
                writeln!(
                    report,
                    "<tr><td>{}</td><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
                    value.row,
                    value.column,
                    html_escape(&value.element)
                )?;
            }
            writeln!(report, "</tbody>")?;
            writeln!(report, "</table>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Schema summary"))?;
            writeln!(report, "<table>")?;
            writeln!(
                report,
                "<thead><tr><th>{}</th><th>{}</th></tr></thead>",
                html_escape("element kind"),
                html_escape("count")
            )?;
            writeln!(report, "<tbody>")?;
            for row in &data.schema_summary {
                writeln!(
                    report,
                    "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
                    html_escape(row.label),
                    row.count
                )?;
            }
            writeln!(report, "</tbody>")?;
            writeln!(report, "</table>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Readiness masks"))?;
            write_shape_flow_table(
                report,
                &[
                    ("none mask", format!("{:?}", data.none_mask_values)),
                    (
                        "numeric mask",
                        format!("strict policy readiness {:?}", data.numeric_mask_values),
                    ),
                    (
                        "strict numeric-ready",
                        data.strict_numeric_ready.to_string(),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Strict conversion"))?;
            write_shape_flow_table(
                report,
                &[("result", data.strict_conversion_result.to_string())],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(
                report,
                "<h2>{}</h2>",
                html_escape("Explicit policy conversion")
            )?;
            write_shape_flow_table(
                report,
                &[
                    ("policy", data.explicit_policy.to_string()),
                    ("converted shape", format!("{:?}", data.converted_shape)),
                    (
                        "converted row-major values",
                        format!("{:?}", data.converted_values),
                    ),
                ],
            )?;
            writeln!(report, "</section>")
        },
    )
}

struct MlprepStandardizationReportData {
    input_shape: Vec<usize>,
    input_values: Vec<f64>,
    before_mean: Vec<f64>,
    before_std: Vec<f64>,
    output_shape: Vec<usize>,
    output_values: Vec<f64>,
    after_mean: Vec<f64>,
    after_std: Vec<f64>,
}

fn mlprep_standardization_report_data() -> Result<MlprepStandardizationReportData, Box<dyn Error>> {
    let input = Tensor::new(vec![8.0, 80.0, 10.0, 100.0, 12.0, 120.0], &[3, 2]);
    let standardized = standardize_columns(&input).map_err(Box::<dyn Error>::from)?;
    let before_mean = input.mean_axis(0);
    let before_std = input.std_axis(0);
    let after_mean = standardized.mean_axis(0);
    let after_std = standardized.std_axis(0);

    Ok(MlprepStandardizationReportData {
        input_shape: input.shape().to_vec(),
        input_values: input.as_slice().to_vec(),
        before_mean: before_mean.as_slice().to_vec(),
        before_std: before_std.as_slice().to_vec(),
        output_shape: standardized.shape().to_vec(),
        output_values: standardized.as_slice().to_vec(),
        after_mean: after_mean.as_slice().to_vec(),
        after_std: after_std.as_slice().to_vec(),
    })
}

pub(crate) fn render_mlprep_standardization_report() -> Result<String, Box<dyn Error>> {
    let data = mlprep_standardization_report_data()?;
    let mut report = String::new();
    writeln!(report, "# matten mlprep-standardization report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_MLPREP_STANDARDIZATION}")?;
    writeln!(
        report,
        "note: fixed demo report, not automatic model-quality analysis"
    )?;
    writeln!(report)?;

    writeln!(report, "## Operation")?;
    writeln!(report, "operation: standardize_columns(input)")?;
    writeln!(
        report,
        "meaning: each column is centered to mean 0 and population standard deviation 1"
    )?;
    writeln!(report)?;

    writeln!(report, "## Before")?;
    writeln!(report, "shape: {:?}", data.input_shape)?;
    writeln!(
        report,
        "row-major values: {}",
        format_fixed_values(&data.input_values)
    )?;
    writeln!(
        report,
        "column mean: {}",
        format_fixed_values(&data.before_mean)
    )?;
    writeln!(
        report,
        "column population std: {}",
        format_fixed_values(&data.before_std)
    )?;
    writeln!(report)?;

    writeln!(report, "## After")?;
    writeln!(report, "shape: {:?}", data.output_shape)?;
    writeln!(
        report,
        "row-major values: {}",
        format_fixed_values(&data.output_values)
    )?;
    writeln!(
        report,
        "column mean: {}",
        format_fixed_values(&data.after_mean)
    )?;
    writeln!(
        report,
        "column population std: {}",
        format_fixed_values(&data.after_std)
    )?;
    writeln!(report)?;

    writeln!(report, "## Shape meaning")?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        data.input_shape, data.output_shape
    )?;
    writeln!(report, "rows: samples unchanged")?;
    writeln!(report, "columns: features unchanged")?;

    Ok(report)
}

pub(crate) fn render_mlprep_standardization_html_report() -> Result<String, Box<dyn Error>> {
    let data = mlprep_standardization_report_data()?;
    render_html_document(
        "matten mlprep-standardization report",
        "Fixed demo report, not automatic model-quality analysis.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Input"))?;
            write_shape_flow_table(
                report,
                &[
                    ("demo", KIND_MLPREP_STANDARDIZATION.to_string()),
                    ("shape", format!("{:?}", data.input_shape)),
                    ("row-major values", format_fixed_values(&data.input_values)),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Operation"))?;
            write_shape_flow_table(
                report,
                &[
                    ("operation", "standardize_columns(input)".to_string()),
                    (
                        "meaning",
                        "each column is centered to mean 0 and population standard deviation 1"
                            .to_string(),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Before"))?;
            write_shape_flow_table(
                report,
                &[
                    ("shape", format!("{:?}", data.input_shape)),
                    ("row-major values", format_fixed_values(&data.input_values)),
                    ("column mean", format_fixed_values(&data.before_mean)),
                    (
                        "column population std",
                        format_fixed_values(&data.before_std),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("After"))?;
            write_shape_flow_table(
                report,
                &[
                    ("shape", format!("{:?}", data.output_shape)),
                    ("row-major values", format_fixed_values(&data.output_values)),
                    ("column mean", format_fixed_values(&data.after_mean)),
                    (
                        "column population std",
                        format_fixed_values(&data.after_std),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Shape meaning"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "shape flow",
                        format!("{:?} -> {:?}", data.input_shape, data.output_shape),
                    ),
                    ("rows", "samples unchanged".to_string()),
                    ("columns", "features unchanged".to_string()),
                ],
            )?;
            writeln!(report, "</section>")
        },
    )
}

struct EducationalPathReportData {
    reading_steps: [&'static str; 4],
    broadcast: EducationalBroadcastData,
    reshape_transpose: EducationalReshapeTransposeData,
    axis_reductions: EducationalAxisReductionData,
    matmul: EducationalMatmulData,
    dynamic_readiness: EducationalDynamicReadinessData,
    standardization: EducationalStandardizationData,
    non_goals: [&'static str; 4],
}

struct EducationalBroadcastData {
    left_shape: Vec<usize>,
    right_shape: Vec<usize>,
    result_shape: Vec<usize>,
    result_values: Vec<f64>,
}

struct EducationalReshapeTransposeData {
    input_shape: Vec<usize>,
    reshape_shape: Vec<usize>,
    reshape_values: Vec<f64>,
    transpose_shape: Vec<usize>,
    transpose_values: Vec<f64>,
}

struct EducationalAxisReductionData {
    input_shape: Vec<usize>,
    mean_axis_0_shape: Vec<usize>,
    mean_axis_0_values: Vec<f64>,
    mean_axis_1_shape: Vec<usize>,
    mean_axis_1_values: Vec<f64>,
}

struct EducationalMatmulData {
    left_shape: Vec<usize>,
    right_shape: Vec<usize>,
    result_shape: Vec<usize>,
    shared_inner_dimension: usize,
    result_values: Vec<f64>,
}

struct EducationalDynamicReadinessData {
    shape: Vec<usize>,
    none_mask_values: Vec<f64>,
    numeric_mask_values: Vec<f64>,
}

struct EducationalStandardizationData {
    input_shape: Vec<usize>,
    output_shape: Vec<usize>,
    before_mean: Vec<f64>,
    before_std: Vec<f64>,
    after_mean: Vec<f64>,
    after_std: Vec<f64>,
}

fn educational_path_report_data() -> Result<EducationalPathReportData, Box<dyn Error>> {
    let broadcast_left = Tensor::new(vec![1.0, 2.0, 3.0], &[3, 1]);
    let broadcast_right = Tensor::new(vec![10.0, 20.0, 30.0, 40.0], &[1, 4]);
    let broadcast = &broadcast_left + &broadcast_right;

    let shape_input = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let reshaped = shape_input.reshape(&[3, 2]);
    let transposed = shape_input.transpose();
    let mean_axis_0 = shape_input.mean_axis(0);
    let mean_axis_1 = shape_input.mean_axis(1);

    let matmul_left = Tensor::new((1..=6).map(|value| value as f64).collect(), &[2, 3]);
    let matmul_right = Tensor::new((1..=12).map(|value| value as f64).collect(), &[3, 4]);
    let matmul = matmul_left.matmul(&matmul_right);

    let dynamic = Tensor::from_elements(
        vec![
            Element::Float(1.0),
            Element::text("2.5"),
            Element::None,
            Element::Int(4),
            Element::text("6.0"),
            Element::Float(8.0),
        ],
        &[2, 3],
    );
    let none_mask = dynamic.none_mask();
    let numeric_mask = dynamic.numeric_mask();

    let standardization_input = Tensor::new(vec![8.0, 80.0, 10.0, 100.0, 12.0, 120.0], &[3, 2]);
    let standardized =
        standardize_columns(&standardization_input).map_err(Box::<dyn Error>::from)?;
    let before_mean = standardization_input.mean_axis(0);
    let before_std = standardization_input.std_axis(0);
    let after_mean = standardized.mean_axis(0);
    let after_std = standardized.std_axis(0);

    Ok(EducationalPathReportData {
        reading_steps: [
            "ask what shape each input has",
            "ask which axes align, disappear, or remain",
            "read the output shape before reading values",
            "convert dynamic data before numeric computation",
        ],
        broadcast: EducationalBroadcastData {
            left_shape: broadcast_left.shape().to_vec(),
            right_shape: broadcast_right.shape().to_vec(),
            result_shape: broadcast.shape().to_vec(),
            result_values: broadcast.as_slice().to_vec(),
        },
        reshape_transpose: EducationalReshapeTransposeData {
            input_shape: shape_input.shape().to_vec(),
            reshape_shape: reshaped.shape().to_vec(),
            reshape_values: reshaped.as_slice().to_vec(),
            transpose_shape: transposed.shape().to_vec(),
            transpose_values: transposed.as_slice().to_vec(),
        },
        axis_reductions: EducationalAxisReductionData {
            input_shape: shape_input.shape().to_vec(),
            mean_axis_0_shape: mean_axis_0.shape().to_vec(),
            mean_axis_0_values: mean_axis_0.as_slice().to_vec(),
            mean_axis_1_shape: mean_axis_1.shape().to_vec(),
            mean_axis_1_values: mean_axis_1.as_slice().to_vec(),
        },
        matmul: EducationalMatmulData {
            left_shape: matmul_left.shape().to_vec(),
            right_shape: matmul_right.shape().to_vec(),
            result_shape: matmul.shape().to_vec(),
            shared_inner_dimension: matmul_left.shape()[1],
            result_values: matmul.as_slice().to_vec(),
        },
        dynamic_readiness: EducationalDynamicReadinessData {
            shape: dynamic.shape().to_vec(),
            none_mask_values: none_mask.as_slice().to_vec(),
            numeric_mask_values: numeric_mask.as_slice().to_vec(),
        },
        standardization: EducationalStandardizationData {
            input_shape: standardization_input.shape().to_vec(),
            output_shape: standardized.shape().to_vec(),
            before_mean: before_mean.as_slice().to_vec(),
            before_std: before_std.as_slice().to_vec(),
            after_mean: after_mean.as_slice().to_vec(),
            after_std: after_std.as_slice().to_vec(),
        },
        non_goals: [
            "not a public API",
            "not source scanning",
            "not a renderer",
            "not model-quality analysis",
        ],
    })
}

pub(crate) fn render_educational_path_report() -> Result<String, Box<dyn Error>> {
    let data = educational_path_report_data()?;
    let mut report = String::new();
    writeln!(report, "# matten educational-path report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_EDUCATIONAL_PATH}")?;
    writeln!(
        report,
        "note: fixed educational demo report, not automatic expression tracing"
    )?;
    writeln!(report)?;

    writeln!(report, "## How to read shapes first")?;
    for (index, step) in data.reading_steps.iter().enumerate() {
        writeln!(report, "{}. {}", index + 1, step)?;
    }
    writeln!(report)?;

    writeln!(report, "## Broadcasting")?;
    writeln!(
        report,
        "shape flow: {:?} + {:?} -> {:?}",
        data.broadcast.left_shape, data.broadcast.right_shape, data.broadcast.result_shape
    )?;
    writeln!(report, "axis 1: left repeats across 4 columns")?;
    writeln!(report, "axis 0: right repeats across 3 rows")?;
    writeln!(report, "result values: {:?}", data.broadcast.result_values)?;
    writeln!(report)?;

    writeln!(report, "## Reshape and transpose")?;
    writeln!(
        report,
        "reshape: {:?} -> {:?}",
        data.reshape_transpose.input_shape, data.reshape_transpose.reshape_shape
    )?;
    writeln!(
        report,
        "reshape values: {:?}",
        data.reshape_transpose.reshape_values
    )?;
    writeln!(
        report,
        "transpose: {:?} -> {:?}",
        data.reshape_transpose.input_shape, data.reshape_transpose.transpose_shape
    )?;
    writeln!(
        report,
        "transpose values: {:?}",
        data.reshape_transpose.transpose_values
    )?;
    writeln!(
        report,
        "meaning: reshape changes grouping; transpose changes coordinate meaning"
    )?;
    writeln!(report)?;

    writeln!(report, "## Axis reductions")?;
    writeln!(
        report,
        "mean_axis(0): {:?} -> {:?}",
        data.axis_reductions.input_shape, data.axis_reductions.mean_axis_0_shape
    )?;
    writeln!(
        report,
        "mean_axis(0) keeps columns: {:?}",
        data.axis_reductions.mean_axis_0_values
    )?;
    writeln!(
        report,
        "mean_axis(1): {:?} -> {:?}",
        data.axis_reductions.input_shape, data.axis_reductions.mean_axis_1_shape
    )?;
    writeln!(
        report,
        "mean_axis(1) keeps rows: {:?}",
        data.axis_reductions.mean_axis_1_values
    )?;
    writeln!(report)?;

    writeln!(report, "## Matrix multiplication")?;
    writeln!(
        report,
        "shape flow: {:?} @ {:?} -> {:?}",
        data.matmul.left_shape, data.matmul.right_shape, data.matmul.result_shape
    )?;
    writeln!(
        report,
        "shared inner dimension: {}",
        data.matmul.shared_inner_dimension
    )?;
    writeln!(report, "result values: {:?}", data.matmul.result_values)?;
    writeln!(report)?;

    writeln!(report, "## Dynamic readiness")?;
    writeln!(report, "dynamic shape: {:?}", data.dynamic_readiness.shape)?;
    writeln!(
        report,
        "none mask: {:?}",
        data.dynamic_readiness.none_mask_values
    )?;
    writeln!(
        report,
        "numeric mask: strict policy readiness {:?}",
        data.dynamic_readiness.numeric_mask_values
    )?;
    writeln!(
        report,
        "Text values are not numeric-ready under the strict mask"
    )?;
    writeln!(report, "next step: clean values, then call try_numeric()")?;
    writeln!(report)?;

    writeln!(report, "## Standardization")?;
    writeln!(report, "operation: standardize_columns(input)")?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        data.standardization.input_shape, data.standardization.output_shape
    )?;
    writeln!(
        report,
        "before column mean: {}",
        format_fixed_values(&data.standardization.before_mean)
    )?;
    writeln!(
        report,
        "before column population std: {}",
        format_fixed_values(&data.standardization.before_std)
    )?;
    writeln!(
        report,
        "after column mean: {}",
        format_fixed_values(&data.standardization.after_mean)
    )?;
    writeln!(
        report,
        "after column population std: {}",
        format_fixed_values(&data.standardization.after_std)
    )?;
    writeln!(report)?;

    writeln!(report, "## What this report is not")?;
    for non_goal in data.non_goals {
        writeln!(report, "- {non_goal}")?;
    }

    Ok(report)
}

pub(crate) fn render_educational_path_html_report() -> Result<String, Box<dyn Error>> {
    let data = educational_path_report_data()?;
    render_html_document(
        "matten educational-path report",
        "Fixed educational demo report, not automatic expression tracing.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(
                report,
                "<h2>{}</h2>",
                html_escape("How to read shapes first")
            )?;
            writeln!(report, "<ol>")?;
            for item in data.reading_steps {
                writeln!(report, "<li>{}</li>", html_escape(item))?;
            }
            writeln!(report, "</ol>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Broadcasting"))?;
            write_shape_flow_table(
                report,
                &[
                    ("left", format!("{:?}", data.broadcast.left_shape)),
                    ("right", format!("{:?}", data.broadcast.right_shape)),
                    ("result", format!("{:?}", data.broadcast.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                html_escape(
                    "axis 1: left repeats across 4 columns; axis 0: right repeats across 3 rows"
                )
            )?;
            write_html_pre(
                report,
                &format!("result values: {:?}", data.broadcast.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Reshape and transpose"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input", format!("{:?}", data.reshape_transpose.input_shape)),
                    (
                        "reshape",
                        format!("{:?}", data.reshape_transpose.reshape_shape),
                    ),
                    (
                        "transpose",
                        format!("{:?}", data.reshape_transpose.transpose_shape),
                    ),
                ],
            )?;
            write_html_pre(
                report,
                &format!(
                    "reshape values: {:?}\ntranspose values: {:?}",
                    data.reshape_transpose.reshape_values, data.reshape_transpose.transpose_values
                ),
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                html_escape("reshape changes grouping; transpose changes coordinate meaning")
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Axis reductions"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "mean_axis(0)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis_reductions.input_shape,
                            data.axis_reductions.mean_axis_0_shape
                        ),
                    ),
                    (
                        "mean_axis(1)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis_reductions.input_shape,
                            data.axis_reductions.mean_axis_1_shape
                        ),
                    ),
                ],
            )?;
            write_html_pre(
                report,
                &format!(
                    "mean_axis(0) keeps columns: {:?}\nmean_axis(1) keeps rows: {:?}",
                    data.axis_reductions.mean_axis_0_values,
                    data.axis_reductions.mean_axis_1_values
                ),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Matrix multiplication"))?;
            write_shape_flow_table(
                report,
                &[
                    ("left", format!("{:?}", data.matmul.left_shape)),
                    ("right", format!("{:?}", data.matmul.right_shape)),
                    ("result", format!("{:?}", data.matmul.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                html_escape(&format!(
                    "shared inner dimension: {}",
                    data.matmul.shared_inner_dimension
                ))
            )?;
            write_html_pre(
                report,
                &format!("result values: {:?}", data.matmul.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Dynamic readiness"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "dynamic shape",
                        format!("{:?}", data.dynamic_readiness.shape),
                    ),
                    (
                        "none mask",
                        format!("{:?}", data.dynamic_readiness.none_mask_values),
                    ),
                    (
                        "numeric mask",
                        format!(
                            "strict policy readiness {:?}",
                            data.dynamic_readiness.numeric_mask_values
                        ),
                    ),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                html_escape(
                    "Text values are not numeric-ready under the strict mask; clean values, then call try_numeric()."
                )
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", html_escape("Standardization"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "shape flow",
                        format!(
                            "{:?} -> {:?}",
                            data.standardization.input_shape, data.standardization.output_shape
                        ),
                    ),
                    (
                        "before mean",
                        format_fixed_values(&data.standardization.before_mean),
                    ),
                    (
                        "before population std",
                        format_fixed_values(&data.standardization.before_std),
                    ),
                    (
                        "after mean",
                        format_fixed_values(&data.standardization.after_mean),
                    ),
                    (
                        "after population std",
                        format_fixed_values(&data.standardization.after_std),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(
                report,
                "<h2>{}</h2>",
                html_escape("What this report is not")
            )?;
            writeln!(report, "<ul>")?;
            for item in data.non_goals {
                writeln!(report, "<li>{}</li>", html_escape(item))?;
            }
            writeln!(report, "</ul>")?;
            writeln!(report, "</section>")?;

            Ok(())
        },
    )
}

fn write_shape_flow_table(
    report: &mut String,
    rows: &[(&str, String)],
) -> Result<(), std::fmt::Error> {
    writeln!(report, "<table>")?;
    writeln!(
        report,
        "<thead><tr><th>{}</th><th>{}</th></tr></thead>",
        html_escape("item"),
        html_escape("shape / value")
    )?;
    writeln!(report, "<tbody>")?;
    for (label, value) in rows {
        writeln!(
            report,
            "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
            html_escape(label),
            html_escape(value)
        )?;
    }
    writeln!(report, "</tbody>")?;
    writeln!(report, "</table>")
}

fn render_html_document<F>(title: &str, note: &str, write_body: F) -> Result<String, Box<dyn Error>>
where
    F: FnOnce(&mut String) -> Result<(), std::fmt::Error>,
{
    let mut report = String::new();
    write_html_document_start(&mut report, title, note)?;
    write_body(&mut report)?;
    write_html_document_end(&mut report)?;
    Ok(report)
}

fn write_html_document_start(
    report: &mut String,
    title: &str,
    note: &str,
) -> Result<(), std::fmt::Error> {
    writeln!(report, "<!doctype html>")?;
    writeln!(report, "<html lang=\"en\">")?;
    writeln!(report, "<head>")?;
    writeln!(report, "  <meta charset=\"utf-8\">")?;
    writeln!(report, "  <title>{}</title>", html_escape(title))?;
    writeln!(report, "  <style>")?;
    writeln!(
        report,
        "    :root {{ color-scheme: light; font-family: system-ui, sans-serif; }}"
    )?;
    writeln!(
        report,
        "    body {{ margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }}"
    )?;
    writeln!(
        report,
        "    h1, h2 {{ color: #14324a; }} section {{ border-top: 1px solid #d6dde5; padding: 1rem 0; }}"
    )?;
    writeln!(
        report,
        "    table {{ width: 100%; border-collapse: collapse; margin: 0.75rem 0; }} th, td {{ border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }}"
    )?;
    writeln!(
        report,
        "    th {{ background: #eef4f8; }} code, .shape {{ font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }}"
    )?;
    writeln!(
        report,
        "    .note {{ background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }}"
    )?;
    writeln!(
        report,
        "    .shape {{ display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }}"
    )?;
    writeln!(report, "  </style>")?;
    writeln!(report, "</head>")?;
    writeln!(report, "<body>")?;
    writeln!(report, "<main>")?;
    writeln!(report, "<h1>{}</h1>", html_escape(title))?;
    writeln!(report, "<p class=\"note\">{}</p>", html_escape(note))
}

fn write_html_document_end(report: &mut String) -> Result<(), std::fmt::Error> {
    writeln!(report, "</main>")?;
    writeln!(report, "</body>")?;
    writeln!(report, "</html>")
}

fn write_html_pre(report: &mut String, value: &str) -> Result<(), std::fmt::Error> {
    writeln!(report, "<pre><code>{}</code></pre>", html_escape(value))
}

fn html_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn format_fixed_values(values: &[f64]) -> String {
    let values = values
        .iter()
        .map(|&value| format_fixed_value(value))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{values}]")
}

fn format_fixed_value(value: f64) -> String {
    let stable = if value.abs() < 0.0005 { 0.0 } else { value };
    format!("{stable:.3}")
}

fn write_list(report: &mut String, values: &[String]) -> Result<(), std::fmt::Error> {
    if values.is_empty() {
        writeln!(report, "- none")?;
    } else {
        for value in values {
            writeln!(report, "- {value}")?;
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests;
