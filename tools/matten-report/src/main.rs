use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use matten::{Element, NumericPolicy, Tensor};
use matten_data::{MattenDataError, Table};
use matten_mlprep::standardize_columns;

const DEMO_CSV: &str = "\
region,sales,cost,note
north,100,40,ok
south,150,45,review
east,120,55,ok";

const KIND_DATA_READINESS: &str = "data-readiness";
const KIND_SHAPE_FLOW: &str = "shape-flow";
const KIND_DYNAMIC_READINESS: &str = "dynamic-readiness";
const KIND_MLPREP_STANDARDIZATION: &str = "mlprep-standardization";
const KIND_EDUCATIONAL_PATH: &str = "educational-path";
const MAX_DISPLAY_COLUMNS: usize = 12;
const MAX_DISPLAY_CHARS: usize = 120;
const MAX_ERROR_CHARS: usize = 240;
const MAX_TENSOR_PREVIEW_VALUES: usize = 12;

#[derive(Debug)]
struct Config {
    input: Input,
    kind: String,
    select: Vec<String>,
    output: Option<PathBuf>,
    format: OutputFormat,
}

#[derive(Debug)]
enum Input {
    Demo { label: String },
    CsvPath { path: PathBuf },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutputFormat {
    Markdown,
    Html,
}

#[derive(Debug)]
enum Action {
    Run(Config),
    Help,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("matten-report error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = match parse_args(env::args().skip(1))? {
        Action::Run(config) => config,
        Action::Help => {
            println!("{}", usage());
            return Ok(());
        }
    };
    let report = render_report(&config)?;

    if let Some(path) = &config.output {
        fs::write(path, report)?;
    } else {
        print!("{report}");
    }

    Ok(())
}

fn parse_args<I>(args: I) -> Result<Action, String>
where
    I: IntoIterator<Item = String>,
{
    let mut demo: Option<String> = None;
    let mut input: Option<PathBuf> = None;
    let mut kind: Option<String> = None;
    let mut select: Option<Vec<String>> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = OutputFormat::Markdown;

    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--demo" => {
                demo = Some(take_value(&mut args, "--demo")?);
            }
            "--input" => {
                input = Some(PathBuf::from(take_value(&mut args, "--input")?));
            }
            "--kind" => {
                kind = Some(take_value(&mut args, "--kind")?);
            }
            "--select" => {
                select = Some(parse_select(&take_value(&mut args, "--select")?)?);
            }
            "--output" => {
                output = Some(PathBuf::from(take_value(&mut args, "--output")?));
            }
            "--format" => {
                format = parse_format(&take_value(&mut args, "--format")?)?;
            }
            "--help" | "-h" => return Ok(Action::Help),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }

    let action = match (demo, input) {
        (Some(label), None) => {
            require_kind_or_demo_label(&label, kind.as_deref())?;
            if select.is_some() {
                return Err(format!(
                    "--select is only accepted with --input; demo mode uses fixed inputs\n\n{}",
                    usage()
                ));
            }
            let select = if label == KIND_DATA_READINESS {
                vec!["sales".to_string(), "cost".to_string()]
            } else {
                Vec::new()
            };
            Ok(Action::Run(Config {
                input: Input::Demo {
                    label: label.clone(),
                },
                kind: label,
                select,
                output,
                format,
            }))
        }
        (None, Some(path)) => {
            let kind =
                kind.ok_or_else(|| format!("--kind is required with --input\n\n{}", usage()))?;
            if kind != KIND_DATA_READINESS {
                return Err(format!(
                    "unsupported --kind {kind:?}; expected {KIND_DATA_READINESS:?}"
                ));
            }
            let select = select
                .ok_or_else(|| format!("--select is required with --input\n\n{}", usage()))?;
            Ok(Action::Run(Config {
                input: Input::CsvPath { path },
                kind,
                select,
                output,
                format,
            }))
        }
        (Some(_), Some(_)) => Err(format!(
            "--demo and --input are mutually exclusive\n\n{}",
            usage()
        )),
        (None, None) => Err(usage()),
    }?;

    if let Action::Run(config) = &action {
        validate_format_policy(config)?;
    }

    Ok(action)
}

fn take_value<I>(args: &mut I, flag: &str) -> Result<String, String>
where
    I: Iterator<Item = String>,
{
    args.next()
        .filter(|value| !value.starts_with("--"))
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn parse_select(value: &str) -> Result<Vec<String>, String> {
    let columns: Vec<String> = value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect();

    if columns.is_empty() {
        Err("--select requires at least one column".to_string())
    } else {
        Ok(columns)
    }
}

fn parse_format(value: &str) -> Result<OutputFormat, String> {
    match value {
        "markdown" => Ok(OutputFormat::Markdown),
        "html" => Ok(OutputFormat::Html),
        other => Err(format!(
            "unsupported --format {other:?}; expected \"markdown\" or \"html\""
        )),
    }
}

fn validate_format_policy(config: &Config) -> Result<(), String> {
    if config.format != OutputFormat::Html {
        return Ok(());
    }

    if config.output.is_none() {
        return Err("--format html requires --output <report.html>".to_string());
    }

    match &config.input {
        Input::Demo { label } if supports_html_demo(label) => Ok(()),
        Input::Demo { label } => Err(format!(
            "--format html is only supported for --demo {}; got {label:?}",
            supported_html_demos()
        )),
        Input::CsvPath { .. } if config.kind == KIND_DATA_READINESS => Ok(()),
        Input::CsvPath { .. } => Err(format!(
            "--format html is only supported for --input <csv-path> --kind {KIND_DATA_READINESS}"
        )),
    }
}

fn supports_html_demo(label: &str) -> bool {
    matches!(
        label,
        KIND_DATA_READINESS
            | KIND_EDUCATIONAL_PATH
            | KIND_SHAPE_FLOW
            | KIND_DYNAMIC_READINESS
            | KIND_MLPREP_STANDARDIZATION
    )
}

fn supported_html_demos() -> &'static str {
    "\"data-readiness\", \"shape-flow\", \"dynamic-readiness\", \"mlprep-standardization\", or \"educational-path\""
}

fn require_kind_or_demo_label(label: &str, kind: Option<&str>) -> Result<(), String> {
    if !is_supported_demo(label) {
        return Err(format!(
            "unsupported --demo {label:?}; expected {KIND_DATA_READINESS:?}, {KIND_SHAPE_FLOW:?}, {KIND_DYNAMIC_READINESS:?}, {KIND_MLPREP_STANDARDIZATION:?}, or {KIND_EDUCATIONAL_PATH:?}"
        ));
    }
    if let Some(kind) = kind {
        if kind != label {
            return Err(format!(
                "unsupported --kind {kind:?} for --demo {label:?}; expected {label:?}"
            ));
        }
    }
    Ok(())
}

fn is_supported_demo(label: &str) -> bool {
    matches!(
        label,
        KIND_DATA_READINESS
            | KIND_SHAPE_FLOW
            | KIND_DYNAMIC_READINESS
            | KIND_MLPREP_STANDARDIZATION
            | KIND_EDUCATIONAL_PATH
    )
}

fn render_report(config: &Config) -> Result<String, Box<dyn Error>> {
    if config.format == OutputFormat::Html {
        return match &config.input {
            Input::Demo { label } if label == KIND_DATA_READINESS => {
                render_data_readiness_html_report()
            }
            Input::Demo { label } if label == KIND_EDUCATIONAL_PATH => {
                render_educational_path_html_report()
            }
            Input::Demo { label } if label == KIND_SHAPE_FLOW => render_shape_flow_html_report(),
            Input::Demo { label } if label == KIND_DYNAMIC_READINESS => {
                render_dynamic_readiness_html_report()
            }
            Input::Demo { label } if label == KIND_MLPREP_STANDARDIZATION => {
                render_mlprep_standardization_html_report()
            }
            Input::Demo { label } => Err(format!(
                "--format html is only supported for --demo {}; got {label:?}",
                supported_html_demos()
            )
            .into()),
            Input::CsvPath { path } => {
                if config.kind != KIND_DATA_READINESS {
                    return Err(format!(
                        "unsupported report kind {:?}; expected {KIND_DATA_READINESS:?}",
                        config.kind
                    )
                    .into());
                }
                let table = Table::from_csv_path(path).map_err(Box::<dyn Error>::from)?;
                render_input_data_readiness_html_report(
                    &format!("path: {}", path.display()),
                    &table,
                    &config.select,
                )
            }
        };
    }

    match &config.input {
        Input::Demo { label } if label == KIND_SHAPE_FLOW => render_shape_flow_report(),
        Input::Demo { label } if label == KIND_DYNAMIC_READINESS => {
            render_dynamic_readiness_report()
        }
        Input::Demo { label } if label == KIND_MLPREP_STANDARDIZATION => {
            render_mlprep_standardization_report()
        }
        Input::Demo { label } if label == KIND_EDUCATIONAL_PATH => render_educational_path_report(),
        Input::Demo { label } if label == KIND_DATA_READINESS => {
            let table = Table::from_csv_str(DEMO_CSV).map_err(Box::<dyn Error>::from)?;
            render_table_report(&format!("demo: {label}"), &table, &config.select)
        }
        Input::Demo { label } => Err(format!("unsupported demo label {label:?}").into()),
        Input::CsvPath { path } => {
            if config.kind != KIND_DATA_READINESS {
                return Err(format!(
                    "unsupported report kind {:?}; expected {KIND_DATA_READINESS:?}",
                    config.kind
                )
                .into());
            }
            let table = Table::from_csv_path(path).map_err(Box::<dyn Error>::from)?;
            render_table_report(&format!("path: {}", path.display()), &table, &config.select)
        }
    }
}

fn render_table_report(
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

fn render_data_readiness_html_report() -> Result<String, Box<dyn Error>> {
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

fn render_input_data_readiness_html_report(
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

fn render_shape_flow_report() -> Result<String, Box<dyn Error>> {
    let data = shape_flow_report_data();
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

struct ShapeFlowReportData {
    broadcast: ShapeFlowBroadcastData,
    reshape: ShapeFlowReshapeData,
    axis: ShapeFlowAxisData,
    matmul: ShapeFlowMatmulData,
}

struct ShapeFlowBroadcastData {
    input_a_shape: Vec<usize>,
    input_b_shape: Vec<usize>,
    result_shape: Vec<usize>,
    operation: &'static str,
    result_values: Vec<f64>,
}

struct ShapeFlowReshapeData {
    input_shape: Vec<usize>,
    result_shape: Vec<usize>,
    operation: &'static str,
    result_values: Vec<f64>,
}

struct ShapeFlowAxisData {
    input_shape: Vec<usize>,
    mean_axis_0_shape: Vec<usize>,
    mean_axis_0_values: Vec<f64>,
    mean_axis_1_shape: Vec<usize>,
    mean_axis_1_values: Vec<f64>,
}

struct ShapeFlowMatmulData {
    left_shape: Vec<usize>,
    right_shape: Vec<usize>,
    result_shape: Vec<usize>,
    operation: &'static str,
    result_values: Vec<f64>,
}

fn shape_flow_report_data() -> ShapeFlowReportData {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);
    let broadcast = &a + &b;
    let reshaped = a.reshape(&[3, 2]);
    let mean_axis_0 = a.mean_axis(0);
    let mean_axis_1 = a.mean_axis(1);
    let left = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let right = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    let product = left.matmul(&right);

    ShapeFlowReportData {
        broadcast: ShapeFlowBroadcastData {
            input_a_shape: a.shape().to_vec(),
            input_b_shape: b.shape().to_vec(),
            result_shape: broadcast.shape().to_vec(),
            operation: "a + b",
            result_values: broadcast.as_slice().to_vec(),
        },
        reshape: ShapeFlowReshapeData {
            input_shape: a.shape().to_vec(),
            result_shape: reshaped.shape().to_vec(),
            operation: "reshape([3, 2])",
            result_values: reshaped.as_slice().to_vec(),
        },
        axis: ShapeFlowAxisData {
            input_shape: a.shape().to_vec(),
            mean_axis_0_shape: mean_axis_0.shape().to_vec(),
            mean_axis_0_values: mean_axis_0.as_slice().to_vec(),
            mean_axis_1_shape: mean_axis_1.shape().to_vec(),
            mean_axis_1_values: mean_axis_1.as_slice().to_vec(),
        },
        matmul: ShapeFlowMatmulData {
            left_shape: left.shape().to_vec(),
            right_shape: right.shape().to_vec(),
            result_shape: product.shape().to_vec(),
            operation: "left.matmul(right)",
            result_values: product.as_slice().to_vec(),
        },
    }
}

fn render_shape_flow_html_report() -> Result<String, Box<dyn Error>> {
    let data = shape_flow_report_data();
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

fn render_dynamic_readiness_report() -> Result<String, Box<dyn Error>> {
    let data = dynamic_readiness_report_data()?;
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

#[derive(Debug)]
struct DynamicReadinessReportData {
    shape: Vec<usize>,
    values: Vec<DynamicValueData>,
    schema_summary: Vec<DynamicSchemaSummaryRow>,
    none_mask_values: Vec<f64>,
    numeric_mask_values: Vec<f64>,
    strict_numeric_ready: bool,
    strict_conversion_result: &'static str,
    explicit_policy: &'static str,
    converted_shape: Vec<usize>,
    converted_values: Vec<f64>,
}

#[derive(Debug)]
struct DynamicValueData {
    row: usize,
    column: usize,
    element: String,
}

#[derive(Debug)]
struct DynamicSchemaSummaryRow {
    label: &'static str,
    count: usize,
}

fn dynamic_readiness_report_data() -> Result<DynamicReadinessReportData, Box<dyn Error>> {
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
    let converted = dynamic
        .try_numeric_with(NumericPolicy::default().none_as(0.0).allow_text_parse())
        .map_err(Box::<dyn Error>::from)?;

    if dynamic.try_numeric().is_ok() {
        return Err("strict dynamic conversion unexpectedly succeeded".into());
    }

    let shape = dynamic.shape().to_vec();
    let columns = shape.get(1).copied().unwrap_or(1);
    let values = dynamic
        .to_elements()
        .iter()
        .enumerate()
        .map(|(index, element)| DynamicValueData {
            row: index / columns,
            column: index % columns,
            element: format_dynamic_element(element),
        })
        .collect();

    Ok(DynamicReadinessReportData {
        shape,
        values,
        schema_summary: dynamic_schema_summary_rows(&dynamic),
        none_mask_values: none_mask.as_slice().to_vec(),
        numeric_mask_values: numeric_mask.as_slice().to_vec(),
        strict_numeric_ready: dynamic.is_numeric_convertible(),
        strict_conversion_result: "error: strict conversion rejects Text and None values",
        explicit_policy: "none_as(0.0) + allow_text_parse()",
        converted_shape: converted.shape().to_vec(),
        converted_values: converted.as_slice().to_vec(),
    })
}

fn dynamic_schema_summary_rows(tensor: &Tensor) -> Vec<DynamicSchemaSummaryRow> {
    let mut floats = 0;
    let mut ints = 0;
    let mut texts = 0;
    let mut bools = 0;
    let mut none = 0;

    for element in tensor.to_elements() {
        match element {
            Element::Float(_) => floats += 1,
            Element::Int(_) => ints += 1,
            Element::Text(_) => texts += 1,
            Element::Bool(_) => bools += 1,
            Element::None => none += 1,
        }
    }

    let mut rows = vec![
        DynamicSchemaSummaryRow {
            label: "Float",
            count: floats,
        },
        DynamicSchemaSummaryRow {
            label: "Int",
            count: ints,
        },
        DynamicSchemaSummaryRow {
            label: "Text",
            count: texts,
        },
    ];
    if bools > 0 {
        rows.push(DynamicSchemaSummaryRow {
            label: "Bool",
            count: bools,
        });
    }
    rows.push(DynamicSchemaSummaryRow {
        label: "None",
        count: none,
    });
    rows
}

fn render_dynamic_readiness_html_report() -> Result<String, Box<dyn Error>> {
    let data = dynamic_readiness_report_data()?;
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

fn render_mlprep_standardization_report() -> Result<String, Box<dyn Error>> {
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

fn render_mlprep_standardization_html_report() -> Result<String, Box<dyn Error>> {
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

fn render_educational_path_report() -> Result<String, Box<dyn Error>> {
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

fn render_educational_path_html_report() -> Result<String, Box<dyn Error>> {
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

fn format_dynamic_element(element: &Element) -> String {
    match element {
        Element::Float(value) => format!("Float({value:?})"),
        Element::Int(value) => format!("Int({value})"),
        Element::Text(value) => format!("Text({value:?})"),
        Element::Bool(value) => format!("Bool({value})"),
        Element::None => "None".to_string(),
    }
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

fn usage() -> String {
    "\
Usage:
  matten-report --demo data-readiness [--output <report.md>]
  matten-report --demo data-readiness --format html --output <report.html>
  matten-report --demo shape-flow [--output <report.md>]
  matten-report --demo shape-flow --format html --output <report.html>
  matten-report --demo dynamic-readiness [--output <report.md>]
  matten-report --demo dynamic-readiness --format html --output <report.html>
  matten-report --demo mlprep-standardization [--output <report.md>]
  matten-report --demo mlprep-standardization --format html --output <report.html>
  matten-report --demo educational-path [--format markdown] [--output <report.md>]
  matten-report --demo educational-path --format html --output <report.html>
  matten-report --input <csv-path> --kind data-readiness --select <col1,col2> [--output <report.md>]
  matten-report --input <csv-path> --kind data-readiness --select <col1,col2> --format html --output <report.html>

Demo reports are fixed examples. Input mode supports only data-readiness.
Markdown is the default format. HTML is local file output and requires --output."
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_CSV: &str = include_str!("../fixtures/small.csv");
    const MISSING_CSV: &str = include_str!("../fixtures/missing.csv");
    const NON_NUMERIC_CSV: &str = include_str!("../fixtures/non_numeric.csv");

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    fn selected(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    fn render_fixture_report(label: &str, csv: &str, values: &[&str]) -> String {
        let table = Table::from_csv_str(csv).expect("fixture CSV should parse");
        render_table_report(label, &table, &selected(values)).expect("report should render")
    }

    #[test]
    fn help_is_success_action() {
        assert!(matches!(parse_args(args(&["--help"])), Ok(Action::Help)));
        assert!(matches!(parse_args(args(&["-h"])), Ok(Action::Help)));
    }

    #[test]
    fn input_mode_requires_kind_and_select() {
        let missing_kind = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--select",
            "sales,cost",
        ]))
        .unwrap_err();
        assert!(missing_kind.contains("--kind is required with --input"));

        let missing_select = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--kind",
            "data-readiness",
        ]))
        .unwrap_err();
        assert!(missing_select.contains("--select is required with --input"));
    }

    #[test]
    fn demo_mode_rejects_select() {
        let err = parse_args(args(&["--demo", "data-readiness", "--select", "sales"])).unwrap_err();
        assert!(err.contains("--select is only accepted with --input"));
    }

    #[test]
    fn demo_shape_flow_allows_output() {
        let action = parse_args(args(&[
            "--demo",
            "shape-flow",
            "--output",
            "target/matten-report-shape-flow.md",
        ]))
        .expect("shape-flow demo with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "shape-flow"
        ));
        assert_eq!(config.kind, "shape-flow");
        assert!(config.select.is_empty());
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-shape-flow.md"))
        );
    }

    #[test]
    fn demo_dynamic_readiness_allows_output() {
        let action = parse_args(args(&[
            "--demo",
            "dynamic-readiness",
            "--output",
            "target/matten-report-dynamic-readiness.md",
        ]))
        .expect("dynamic-readiness demo with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "dynamic-readiness"
        ));
        assert_eq!(config.kind, "dynamic-readiness");
        assert!(config.select.is_empty());
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-dynamic-readiness.md"))
        );
    }

    #[test]
    fn demo_mlprep_standardization_allows_output() {
        let action = parse_args(args(&[
            "--demo",
            "mlprep-standardization",
            "--output",
            "target/matten-report-mlprep-standardization.md",
        ]))
        .expect("mlprep-standardization demo with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "mlprep-standardization"
        ));
        assert_eq!(config.kind, "mlprep-standardization");
        assert!(config.select.is_empty());
        assert_eq!(
            config.output,
            Some(PathBuf::from(
                "target/matten-report-mlprep-standardization.md"
            ))
        );
    }

    #[test]
    fn demo_educational_path_allows_kind_and_output() {
        let action = parse_args(args(&[
            "--demo",
            "educational-path",
            "--kind",
            "educational-path",
            "--output",
            "target/matten-report-educational-path.md",
        ]))
        .expect("educational-path demo with matching kind and output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "educational-path"
        ));
        assert_eq!(config.kind, "educational-path");
        assert!(config.select.is_empty());
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-educational-path.md"))
        );
        assert_eq!(config.format, OutputFormat::Markdown);
    }

    #[test]
    fn educational_path_html_requires_output() {
        let err =
            parse_args(args(&["--demo", "educational-path", "--format", "html"])).unwrap_err();

        assert!(err.contains("--format html requires --output <report.html>"));
    }

    #[test]
    fn educational_path_html_allows_explicit_output() {
        let action = parse_args(args(&[
            "--demo",
            "educational-path",
            "--format",
            "html",
            "--output",
            "target/matten-report-educational-path.html",
        ]))
        .expect("educational-path HTML with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "educational-path"
        ));
        assert_eq!(config.format, OutputFormat::Html);
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-educational-path.html"))
        );
    }

    #[test]
    fn data_readiness_html_requires_output() {
        let err = parse_args(args(&["--demo", "data-readiness", "--format", "html"])).unwrap_err();

        assert!(err.contains("--format html requires --output <report.html>"));
    }

    #[test]
    fn data_readiness_html_allows_explicit_output() {
        let action = parse_args(args(&[
            "--demo",
            "data-readiness",
            "--format",
            "html",
            "--output",
            "target/matten-report-data-readiness.html",
        ]))
        .expect("data-readiness HTML with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "data-readiness"
        ));
        assert_eq!(config.format, OutputFormat::Html);
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-data-readiness.html"))
        );
    }

    #[test]
    fn shape_flow_html_requires_output() {
        let err = parse_args(args(&["--demo", "shape-flow", "--format", "html"])).unwrap_err();

        assert!(err.contains("--format html requires --output <report.html>"));
    }

    #[test]
    fn shape_flow_html_allows_explicit_output() {
        let action = parse_args(args(&[
            "--demo",
            "shape-flow",
            "--format",
            "html",
            "--output",
            "target/matten-report-shape-flow.html",
        ]))
        .expect("shape-flow HTML with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "shape-flow"
        ));
        assert_eq!(config.format, OutputFormat::Html);
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-shape-flow.html"))
        );
    }

    #[test]
    fn dynamic_readiness_html_requires_output() {
        let err =
            parse_args(args(&["--demo", "dynamic-readiness", "--format", "html"])).unwrap_err();

        assert!(err.contains("--format html requires --output <report.html>"));
    }

    #[test]
    fn dynamic_readiness_html_allows_explicit_output() {
        let action = parse_args(args(&[
            "--demo",
            "dynamic-readiness",
            "--format",
            "html",
            "--output",
            "target/matten-report-dynamic-readiness.html",
        ]))
        .expect("dynamic-readiness HTML with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "dynamic-readiness"
        ));
        assert_eq!(config.format, OutputFormat::Html);
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-dynamic-readiness.html"))
        );
    }

    #[test]
    fn mlprep_standardization_html_requires_output() {
        let err = parse_args(args(&[
            "--demo",
            "mlprep-standardization",
            "--format",
            "html",
        ]))
        .unwrap_err();

        assert!(err.contains("--format html requires --output <report.html>"));
    }

    #[test]
    fn mlprep_standardization_html_allows_explicit_output() {
        let action = parse_args(args(&[
            "--demo",
            "mlprep-standardization",
            "--format",
            "html",
            "--output",
            "target/matten-report-mlprep-standardization.html",
        ]))
        .expect("mlprep-standardization HTML with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(
            config.input,
            Input::Demo { ref label } if label == "mlprep-standardization"
        ));
        assert_eq!(config.format, OutputFormat::Html);
        assert_eq!(
            config.output,
            Some(PathBuf::from(
                "target/matten-report-mlprep-standardization.html"
            ))
        );
    }

    #[test]
    fn input_mode_html_requires_output() {
        let err = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--kind",
            "data-readiness",
            "--select",
            "sales,cost",
            "--format",
            "html",
        ]))
        .unwrap_err();

        assert!(err.contains("--format html requires --output <report.html>"));
    }

    #[test]
    fn input_mode_html_allows_explicit_output() {
        let action = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--kind",
            "data-readiness",
            "--select",
            "sales,cost",
            "--format",
            "html",
            "--output",
            "target/matten-report-data-readiness.html",
        ]))
        .expect("input-mode data-readiness HTML with output should parse");

        let Action::Run(config) = action else {
            panic!("expected run action");
        };
        assert!(matches!(config.input, Input::CsvPath { .. }));
        assert_eq!(config.kind, "data-readiness");
        assert_eq!(config.select, selected(&["sales", "cost"]));
        assert_eq!(config.format, OutputFormat::Html);
        assert_eq!(
            config.output,
            Some(PathBuf::from("target/matten-report-data-readiness.html"))
        );
    }

    #[test]
    fn unknown_format_is_rejected() {
        let err = parse_args(args(&[
            "--demo",
            "educational-path",
            "--format",
            "svg",
            "--output",
            "target/matten-report-educational-path.svg",
        ]))
        .unwrap_err();

        assert!(err.contains("unsupported --format \"svg\"; expected \"markdown\" or \"html\""));
    }

    #[test]
    fn shape_flow_input_mode_is_not_supported() {
        let err = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--kind",
            "shape-flow",
            "--select",
            "sales,cost",
        ]))
        .unwrap_err();

        assert!(err.contains("unsupported --kind \"shape-flow\"; expected \"data-readiness\""));
    }

    #[test]
    fn dynamic_readiness_input_mode_is_not_supported() {
        let err = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--kind",
            "dynamic-readiness",
            "--select",
            "sales,cost",
        ]))
        .unwrap_err();

        assert!(
            err.contains("unsupported --kind \"dynamic-readiness\"; expected \"data-readiness\"")
        );
    }

    #[test]
    fn mlprep_standardization_input_mode_is_not_supported() {
        let err = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--kind",
            "mlprep-standardization",
            "--select",
            "sales,cost",
        ]))
        .unwrap_err();

        assert!(err.contains(
            "unsupported --kind \"mlprep-standardization\"; expected \"data-readiness\""
        ));
    }

    #[test]
    fn educational_path_input_mode_is_not_supported() {
        let err = parse_args(args(&[
            "--input",
            "fixtures/small.csv",
            "--kind",
            "educational-path",
            "--select",
            "sales,cost",
        ]))
        .unwrap_err();

        assert!(
            err.contains("unsupported --kind \"educational-path\"; expected \"data-readiness\"")
        );
    }

    #[test]
    fn educational_path_demo_rejects_select() {
        let err =
            parse_args(args(&["--demo", "educational-path", "--select", "sales"])).unwrap_err();

        assert!(err.contains("--select is only accepted with --input"));
    }

    #[test]
    fn unsupported_demo_label_remains_readable() {
        let err = parse_args(args(&["--demo", "unknown"])).unwrap_err();

        assert!(err.contains("unsupported --demo \"unknown\"; expected \"data-readiness\", \"shape-flow\", \"dynamic-readiness\", \"mlprep-standardization\", or \"educational-path\""));
    }

    #[test]
    fn shape_flow_report_still_matches_expected_markdown() {
        let report = render_shape_flow_report().expect("shape-flow report should render");

        assert_eq!(
            report,
            "\
# matten shape-flow report

## Input
demo: shape-flow
note: fixed demo report, not automatic expression tracing

## Broadcast add
input a: shape [2, 3]
input b: shape [3]
operation: a + b
shape flow: [2, 3] + [3] -> [2, 3]
result values: [11.0, 22.0, 33.0, 14.0, 25.0, 36.0]

## Reshape
input: shape [2, 3]
operation: reshape([3, 2])
shape flow: [2, 3] -> [3, 2]
result values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]

## Axis reductions
input: shape [2, 3]
mean_axis(0): [2, 3] -> [3]
mean_axis(0) values: [2.5, 3.5, 4.5]
mean_axis(1): [2, 3] -> [2]
mean_axis(1) values: [2.0, 5.0]

## Matrix multiplication
left: shape [2, 3]
right: shape [3, 2]
operation: left.matmul(right)
shape flow: [2, 3] @ [3, 2] -> [2, 2]
result values: [22.0, 28.0, 49.0, 64.0]
"
        );
    }

    #[test]
    fn shape_flow_html_report_matches_expected_html() {
        let report = render_shape_flow_html_report().expect("shape-flow HTML should render");

        assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten shape-flow report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten shape-flow report</h1>
<p class=\"note\">Fixed demo report, not automatic expression tracing.</p>
<section>
<h2>Broadcast add</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input a</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>input b</td><td><span class=\"shape\">[3]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[2, 3]</span></td></tr>
</tbody>
</table>
<p>operation: a + b</p>
<pre><code>result values: [11.0, 22.0, 33.0, 14.0, 25.0, 36.0]</code></pre>
</section>
<section>
<h2>Reshape</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[3, 2]</span></td></tr>
</tbody>
</table>
<p>operation: reshape([3, 2])</p>
<pre><code>result values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]</code></pre>
</section>
<section>
<h2>Axis reductions</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>mean_axis(0)</td><td><span class=\"shape\">[2, 3] -&gt; [3]</span></td></tr>
<tr><td>mean_axis(1)</td><td><span class=\"shape\">[2, 3] -&gt; [2]</span></td></tr>
</tbody>
</table>
<pre><code>mean_axis(0) values: [2.5, 3.5, 4.5]
mean_axis(1) values: [2.0, 5.0]</code></pre>
</section>
<section>
<h2>Matrix multiplication</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>left</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>right</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[2, 2]</span></td></tr>
</tbody>
</table>
<p>operation: left.matmul(right)</p>
<pre><code>result values: [22.0, 28.0, 49.0, 64.0]</code></pre>
</section>
</main>
</body>
</html>
"
        );
    }

    #[test]
    fn shape_flow_html_report_is_static_and_self_contained() {
        let report = render_shape_flow_html_report().expect("shape-flow HTML should render");

        assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
        assert!(report.contains("<title>matten shape-flow report</title>"));
        assert!(report.contains("<h1>matten shape-flow report</h1>"));
        assert!(report.contains("<h2>Broadcast add</h2>"));
        assert!(report.contains("<span class=\"shape\">[2, 3]</span>"));
        assert!(report.contains("<h2>Axis reductions</h2>"));
        assert!(report.contains("[2, 3] -&gt; [3]"));
        assert!(report.contains("<h2>Matrix multiplication</h2>"));
        assert!(report.contains("result values: [22.0, 28.0, 49.0, 64.0]"));
        assert!(!report.contains("<script"));
        assert!(!report.contains(" src="));
        assert!(!report.contains(" href="));
        assert!(!report.contains("data:"));
        assert!(!report.contains("<svg"));
    }

    #[test]
    fn dynamic_readiness_report_matches_expected_markdown() {
        let report =
            render_dynamic_readiness_report().expect("dynamic-readiness report should render");

        assert_eq!(
            report,
            "\
# matten dynamic-readiness report

## Input
demo: dynamic-readiness
note: fixed demo report, not automatic data profiling

## Dynamic values
shape: [2, 3]
row-major values:
- [0, 0] Float(1.0)
- [0, 1] Text(\"2.5\")
- [0, 2] None
- [1, 0] Int(4)
- [1, 1] Text(\"6.0\")
- [1, 2] Float(8.0)
schema summary:
- Float: 2
- Int: 1
- Text: 2
- None: 1

## Readiness masks
none mask: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
numeric mask: strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]
strict numeric-ready: false

## Strict conversion
result: error: strict conversion rejects Text and None values

## Explicit policy conversion
policy: none_as(0.0) + allow_text_parse()
converted shape: [2, 3]
converted row-major values: [1.0, 2.5, 0.0, 4.0, 6.0, 8.0]
"
        );
    }

    #[test]
    fn dynamic_readiness_html_report_matches_expected_html() {
        let report =
            render_dynamic_readiness_html_report().expect("dynamic-readiness HTML should render");

        assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten dynamic-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten dynamic-readiness report</h1>
<p class=\"note\">Fixed demo report, not automatic data profiling.</p>
<section>
<h2>Dynamic values</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[2, 3]</span></td></tr>
</tbody>
</table>
<table>
<thead><tr><th>row</th><th>column</th><th>value</th></tr></thead>
<tbody>
<tr><td>0</td><td>0</td><td><span class=\"shape\">Float(1.0)</span></td></tr>
<tr><td>0</td><td>1</td><td><span class=\"shape\">Text(&quot;2.5&quot;)</span></td></tr>
<tr><td>0</td><td>2</td><td><span class=\"shape\">None</span></td></tr>
<tr><td>1</td><td>0</td><td><span class=\"shape\">Int(4)</span></td></tr>
<tr><td>1</td><td>1</td><td><span class=\"shape\">Text(&quot;6.0&quot;)</span></td></tr>
<tr><td>1</td><td>2</td><td><span class=\"shape\">Float(8.0)</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Schema summary</h2>
<table>
<thead><tr><th>element kind</th><th>count</th></tr></thead>
<tbody>
<tr><td>Float</td><td><span class=\"shape\">2</span></td></tr>
<tr><td>Int</td><td><span class=\"shape\">1</span></td></tr>
<tr><td>Text</td><td><span class=\"shape\">2</span></td></tr>
<tr><td>None</td><td><span class=\"shape\">1</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Readiness masks</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>none mask</td><td><span class=\"shape\">[0.0, 0.0, 1.0, 0.0, 0.0, 0.0]</span></td></tr>
<tr><td>numeric mask</td><td><span class=\"shape\">strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]</span></td></tr>
<tr><td>strict numeric-ready</td><td><span class=\"shape\">false</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Strict conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>result</td><td><span class=\"shape\">error: strict conversion rejects Text and None values</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Explicit policy conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>policy</td><td><span class=\"shape\">none_as(0.0) + allow_text_parse()</span></td></tr>
<tr><td>converted shape</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>converted row-major values</td><td><span class=\"shape\">[1.0, 2.5, 0.0, 4.0, 6.0, 8.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
    }

    #[test]
    fn dynamic_readiness_html_report_is_static_and_self_contained() {
        let report =
            render_dynamic_readiness_html_report().expect("dynamic-readiness HTML should render");

        assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
        assert!(report.contains("<title>matten dynamic-readiness report</title>"));
        assert!(report.contains("<h1>matten dynamic-readiness report</h1>"));
        assert!(report.contains("<h2>Dynamic values</h2>"));
        assert!(report.contains("<span class=\"shape\">Text(&quot;2.5&quot;)</span>"));
        assert!(report.contains("<h2>Readiness masks</h2>"));
        assert!(report.contains("strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]"));
        assert!(report.contains("<h2>Strict conversion</h2>"));
        assert!(report.contains("error: strict conversion rejects Text and None values"));
        assert!(report.contains("<h2>Explicit policy conversion</h2>"));
        assert!(report.contains("[1.0, 2.5, 0.0, 4.0, 6.0, 8.0]"));
        assert!(!report.contains("<script"));
        assert!(!report.contains(" src="));
        assert!(!report.contains(" href="));
        assert!(!report.contains("data:"));
        assert!(!report.contains("<svg"));
    }

    #[test]
    fn mlprep_standardization_report_matches_expected_markdown() {
        let report = render_mlprep_standardization_report()
            .expect("mlprep-standardization report should render");

        assert_eq!(
            report,
            "\
# matten mlprep-standardization report

## Input
demo: mlprep-standardization
note: fixed demo report, not automatic model-quality analysis

## Operation
operation: standardize_columns(input)
meaning: each column is centered to mean 0 and population standard deviation 1

## Before
shape: [3, 2]
row-major values: [8.000, 80.000, 10.000, 100.000, 12.000, 120.000]
column mean: [10.000, 100.000]
column population std: [1.633, 16.330]

## After
shape: [3, 2]
row-major values: [-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]
column mean: [0.000, 0.000]
column population std: [1.000, 1.000]

## Shape meaning
shape flow: [3, 2] -> [3, 2]
rows: samples unchanged
columns: features unchanged
"
        );
    }

    #[test]
    fn mlprep_standardization_html_report_matches_expected_html() {
        let report = render_mlprep_standardization_html_report()
            .expect("mlprep-standardization HTML should render");

        assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten mlprep-standardization report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten mlprep-standardization report</h1>
<p class=\"note\">Fixed demo report, not automatic model-quality analysis.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>demo</td><td><span class=\"shape\">mlprep-standardization</span></td></tr>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[8.000, 80.000, 10.000, 100.000, 12.000, 120.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Operation</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>operation</td><td><span class=\"shape\">standardize_columns(input)</span></td></tr>
<tr><td>meaning</td><td><span class=\"shape\">each column is centered to mean 0 and population standard deviation 1</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Before</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[8.000, 80.000, 10.000, 100.000, 12.000, 120.000]</span></td></tr>
<tr><td>column mean</td><td><span class=\"shape\">[10.000, 100.000]</span></td></tr>
<tr><td>column population std</td><td><span class=\"shape\">[1.633, 16.330]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>After</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]</span></td></tr>
<tr><td>column mean</td><td><span class=\"shape\">[0.000, 0.000]</span></td></tr>
<tr><td>column population std</td><td><span class=\"shape\">[1.000, 1.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Shape meaning</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape flow</td><td><span class=\"shape\">[3, 2] -&gt; [3, 2]</span></td></tr>
<tr><td>rows</td><td><span class=\"shape\">samples unchanged</span></td></tr>
<tr><td>columns</td><td><span class=\"shape\">features unchanged</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
    }

    #[test]
    fn mlprep_standardization_html_report_is_static_and_self_contained() {
        let report = render_mlprep_standardization_html_report()
            .expect("mlprep-standardization HTML should render");

        assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
        assert!(report.contains("<title>matten mlprep-standardization report</title>"));
        assert!(report.contains("<h1>matten mlprep-standardization report</h1>"));
        assert!(report.contains("not automatic model-quality analysis"));
        assert!(report.contains("<h2>Input</h2>"));
        assert!(report.contains("<h2>Operation</h2>"));
        assert!(report.contains("standardize_columns(input)"));
        assert!(report.contains("<h2>Before</h2>"));
        assert!(report.contains("[10.000, 100.000]"));
        assert!(report.contains("[1.633, 16.330]"));
        assert!(report.contains("<h2>After</h2>"));
        assert!(report.contains("[-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]"));
        assert!(report.contains("[0.000, 0.000]"));
        assert!(report.contains("[1.000, 1.000]"));
        assert!(report.contains("<h2>Shape meaning</h2>"));
        assert!(report.contains("[3, 2] -&gt; [3, 2]"));
        assert!(!report.contains("<script"));
        assert!(!report.contains(" src="));
        assert!(!report.contains(" href="));
        assert!(!report.contains("data:"));
        assert!(!report.contains("<svg"));
    }

    #[test]
    fn educational_path_report_matches_expected_markdown() {
        let report =
            render_educational_path_report().expect("educational-path report should render");

        assert_eq!(
            report,
            "\
# matten educational-path report

## Input
demo: educational-path
note: fixed educational demo report, not automatic expression tracing

## How to read shapes first
1. ask what shape each input has
2. ask which axes align, disappear, or remain
3. read the output shape before reading values
4. convert dynamic data before numeric computation

## Broadcasting
shape flow: [3, 1] + [1, 4] -> [3, 4]
axis 1: left repeats across 4 columns
axis 0: right repeats across 3 rows
result values: [11.0, 21.0, 31.0, 41.0, 12.0, 22.0, 32.0, 42.0, 13.0, 23.0, 33.0, 43.0]

## Reshape and transpose
reshape: [2, 3] -> [3, 2]
reshape values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
transpose: [2, 3] -> [3, 2]
transpose values: [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]
meaning: reshape changes grouping; transpose changes coordinate meaning

## Axis reductions
mean_axis(0): [2, 3] -> [3]
mean_axis(0) keeps columns: [2.5, 3.5, 4.5]
mean_axis(1): [2, 3] -> [2]
mean_axis(1) keeps rows: [2.0, 5.0]

## Matrix multiplication
shape flow: [2, 3] @ [3, 4] -> [2, 4]
shared inner dimension: 3
result values: [38.0, 44.0, 50.0, 56.0, 83.0, 98.0, 113.0, 128.0]

## Dynamic readiness
dynamic shape: [2, 3]
none mask: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
numeric mask: strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]
Text values are not numeric-ready under the strict mask
next step: clean values, then call try_numeric()

## Standardization
operation: standardize_columns(input)
shape flow: [3, 2] -> [3, 2]
before column mean: [10.000, 100.000]
before column population std: [1.633, 16.330]
after column mean: [0.000, 0.000]
after column population std: [1.000, 1.000]

## What this report is not
- not a public API
- not source scanning
- not a renderer
- not model-quality analysis
"
        );
    }

    #[test]
    fn educational_path_html_report_matches_expected_html() {
        let report =
            render_educational_path_html_report().expect("educational-path HTML should render");

        assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten educational-path report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten educational-path report</h1>
<p class=\"note\">Fixed educational demo report, not automatic expression tracing.</p>
<section>
<h2>How to read shapes first</h2>
<ol>
<li>ask what shape each input has</li>
<li>ask which axes align, disappear, or remain</li>
<li>read the output shape before reading values</li>
<li>convert dynamic data before numeric computation</li>
</ol>
</section>
<section>
<h2>Broadcasting</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>left</td><td><span class=\"shape\">[3, 1]</span></td></tr>
<tr><td>right</td><td><span class=\"shape\">[1, 4]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[3, 4]</span></td></tr>
</tbody>
</table>
<p>axis 1: left repeats across 4 columns; axis 0: right repeats across 3 rows</p>
<pre><code>result values: [11.0, 21.0, 31.0, 41.0, 12.0, 22.0, 32.0, 42.0, 13.0, 23.0, 33.0, 43.0]</code></pre>
</section>
<section>
<h2>Reshape and transpose</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>reshape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>transpose</td><td><span class=\"shape\">[3, 2]</span></td></tr>
</tbody>
</table>
<pre><code>reshape values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
transpose values: [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]</code></pre>
<p>reshape changes grouping; transpose changes coordinate meaning</p>
</section>
<section>
<h2>Axis reductions</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>mean_axis(0)</td><td><span class=\"shape\">[2, 3] -&gt; [3]</span></td></tr>
<tr><td>mean_axis(1)</td><td><span class=\"shape\">[2, 3] -&gt; [2]</span></td></tr>
</tbody>
</table>
<pre><code>mean_axis(0) keeps columns: [2.5, 3.5, 4.5]
mean_axis(1) keeps rows: [2.0, 5.0]</code></pre>
</section>
<section>
<h2>Matrix multiplication</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>left</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>right</td><td><span class=\"shape\">[3, 4]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[2, 4]</span></td></tr>
</tbody>
</table>
<p>shared inner dimension: 3</p>
<pre><code>result values: [38.0, 44.0, 50.0, 56.0, 83.0, 98.0, 113.0, 128.0]</code></pre>
</section>
<section>
<h2>Dynamic readiness</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>dynamic shape</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>none mask</td><td><span class=\"shape\">[0.0, 0.0, 1.0, 0.0, 0.0, 0.0]</span></td></tr>
<tr><td>numeric mask</td><td><span class=\"shape\">strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]</span></td></tr>
</tbody>
</table>
<p>Text values are not numeric-ready under the strict mask; clean values, then call try_numeric().</p>
</section>
<section>
<h2>Standardization</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape flow</td><td><span class=\"shape\">[3, 2] -&gt; [3, 2]</span></td></tr>
<tr><td>before mean</td><td><span class=\"shape\">[10.000, 100.000]</span></td></tr>
<tr><td>before population std</td><td><span class=\"shape\">[1.633, 16.330]</span></td></tr>
<tr><td>after mean</td><td><span class=\"shape\">[0.000, 0.000]</span></td></tr>
<tr><td>after population std</td><td><span class=\"shape\">[1.000, 1.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>What this report is not</h2>
<ul>
<li>not a public API</li>
<li>not source scanning</li>
<li>not a renderer</li>
<li>not model-quality analysis</li>
</ul>
</section>
</main>
</body>
</html>
"
        );
    }

    #[test]
    fn educational_path_html_report_is_static_and_self_contained() {
        let report =
            render_educational_path_html_report().expect("educational-path HTML should render");

        assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
        assert!(report.contains("<title>matten educational-path report</title>"));
        assert!(report.contains("<h1>matten educational-path report</h1>"));
        assert!(report.contains("<h2>Broadcasting</h2>"));
        assert!(report.contains("<span class=\"shape\">[3, 1]</span>"));
        assert!(report.contains("<h2>Dynamic readiness</h2>"));
        assert!(report.contains("strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]"));
        assert!(report.contains("<h2>Standardization</h2>"));
        assert!(report.contains("after population std"));
        assert!(!report.contains("<script"));
        assert!(!report.contains(" src="));
        assert!(!report.contains(" href="));
        assert!(!report.contains("data:"));
        assert!(!report.contains("<svg"));
    }

    #[test]
    fn data_readiness_report_still_matches_expected_markdown() {
        let report = render_fixture_report("fixture: small.csv", SMALL_CSV, &["sales", "cost"]);

        assert_eq!(
            report,
            "\
# matten data-readiness report

## Input
fixture: small.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: success

## Tensor preview
shape: [3, 2]
row-major values: [100.0, 40.0, 150.0, 45.0, 120.0, 55.0]
"
        );
    }

    #[test]
    fn data_readiness_demo_report_matches_expected_markdown() {
        let report = render_report(&Config {
            input: Input::Demo {
                label: KIND_DATA_READINESS.to_string(),
            },
            kind: KIND_DATA_READINESS.to_string(),
            select: selected(&["sales", "cost"]),
            output: None,
            format: OutputFormat::Markdown,
        })
        .expect("data-readiness demo report should render");

        assert_eq!(
            report,
            "\
# matten data-readiness report

## Input
demo: data-readiness

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: success

## Tensor preview
shape: [3, 2]
row-major values: [100.0, 40.0, 150.0, 45.0, 120.0, 55.0]
"
        );
    }

    #[test]
    fn data_readiness_html_report_matches_expected_html() {
        let report =
            render_data_readiness_html_report().expect("data-readiness HTML should render");

        assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten data-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten data-readiness report</h1>
<p class=\"note\">Fixed demo report, not arbitrary CSV profiling.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">demo: data-readiness</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Columns</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>source columns</td><td><span class=\"shape\">region, sales, cost, note</span></td></tr>
<tr><td>selected columns</td><td><span class=\"shape\">sales, cost</span></td></tr>
<tr><td>columns left out</td><td><span class=\"shape\">region, note</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Missing values</h2>
<table>
<thead><tr><th>column</th><th>missing</th></tr></thead>
<tbody>
<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>
<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Numeric conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>strict conversion</td><td><span class=\"shape\">success</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Tensor preview</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
    }

    #[test]
    fn data_readiness_html_report_is_static_and_self_contained() {
        let report =
            render_data_readiness_html_report().expect("data-readiness HTML should render");

        assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
        assert!(report.contains("<title>matten data-readiness report</title>"));
        assert!(report.contains("<h1>matten data-readiness report</h1>"));
        assert!(report.contains("Fixed demo report, not arbitrary CSV profiling."));
        assert!(report.contains("<h2>Columns</h2>"));
        assert!(report.contains("region, sales, cost, note"));
        assert!(report.contains("sales, cost"));
        assert!(report.contains("region, note"));
        assert!(report.contains("<h2>Missing values</h2>"));
        assert!(report.contains("<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>"));
        assert!(report.contains("<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>"));
        assert!(report.contains("<h2>Numeric conversion</h2>"));
        assert!(report.contains("<span class=\"shape\">success</span>"));
        assert!(report.contains("<h2>Tensor preview</h2>"));
        assert!(report.contains("<span class=\"shape\">[3, 2]</span>"));
        assert!(report.contains("[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]"));
        assert!(!report.contains("<script"));
        assert!(!report.contains(" src="));
        assert!(!report.contains(" href="));
        assert!(!report.contains("data:"));
        assert!(!report.contains("<svg"));
    }

    #[test]
    fn input_data_readiness_html_success_matches_expected_html() {
        let table = Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");
        let report = render_input_data_readiness_html_report(
            "path: tools/matten-report/fixtures/small.csv",
            &table,
            &selected(&["sales", "cost"]),
        )
        .expect("input-mode data-readiness HTML should render");

        assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten data-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten data-readiness report</h1>
<p class=\"note\">Bounded summary of the provided CSV file; not a full raw table rendering.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">path: tools/matten-report/fixtures/small.csv</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Columns</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>source columns</td><td><span class=\"shape\">region, sales, cost, note</span></td></tr>
<tr><td>selected columns</td><td><span class=\"shape\">sales, cost</span></td></tr>
<tr><td>columns left out</td><td><span class=\"shape\">region, note</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Missing values</h2>
<table>
<thead><tr><th>column</th><th>missing</th></tr></thead>
<tbody>
<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>
<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Numeric conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>strict conversion</td><td><span class=\"shape\">success</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Tensor preview</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
    }

    #[test]
    fn input_data_readiness_html_error_is_bounded_summary() {
        let table = Table::from_csv_str(NON_NUMERIC_CSV).expect("fixture CSV should parse");
        let report = render_input_data_readiness_html_report(
            "path: tools/matten-report/fixtures/non_numeric.csv",
            &table,
            &selected(&["sales", "cost"]),
        )
        .expect("input-mode data-readiness error HTML should render");

        assert!(report.contains("<h1>matten data-readiness report</h1>"));
        assert!(report.contains("Bounded summary of the provided CSV file"));
        assert!(report.contains("<h2>Numeric conversion</h2>"));
        assert!(report.contains("<span class=\"shape\">error</span>"));
        assert!(report.contains(
            "non-numeric value &quot;oops&quot; in column &quot;sales&quot;, CSV line 3"
        ));
        assert!(!report.contains("<h2>Tensor preview</h2>"));
        assert!(!report.contains("Fixed demo report, not arbitrary CSV profiling."));
    }

    #[test]
    fn input_data_readiness_html_is_static_self_contained_and_escaped() {
        let csv = "\
region,<script>alert(1)</script>,cost,note
north,<b>oops</b>,40,ok
";
        let table = Table::from_csv_str(csv).expect("hostile fixture CSV should parse");
        let report = render_input_data_readiness_html_report(
            "path: <script>/tmp/hostile.csv</script>",
            &table,
            &selected(&["<script>alert(1)</script>", "cost"]),
        )
        .expect("hostile input HTML should render");

        assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
        assert!(report.contains("path: &lt;script&gt;/tmp/hostile.csv&lt;/script&gt;"));
        assert!(report.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
        assert!(report.contains("&lt;b&gt;oops&lt;/b&gt;"));
        assert!(!report.contains("<script>alert(1)</script>"));
        assert!(!report.contains("<b>oops</b>"));
        assert!(!report.contains("<script"));
        assert!(!report.contains(" src="));
        assert!(!report.contains(" href="));
        assert!(!report.contains("data:"));
        assert!(!report.contains("<svg"));
    }

    #[test]
    fn input_data_readiness_html_bounds_wide_and_long_fields() {
        let headers: Vec<String> = (0..15)
            .map(|index| {
                if index == 1 {
                    format!("selected_{}", "x".repeat(180))
                } else {
                    format!("col{index}")
                }
            })
            .collect();
        let values: Vec<String> = (0..15).map(|index| index.to_string()).collect();
        let csv = format!("{}\n{}\n", headers.join(","), values.join(","));
        let table = Table::from_csv_str(&csv).expect("wide fixture CSV should parse");
        let report = render_input_data_readiness_html_report(
            &format!("path: {}", "p".repeat(180)),
            &table,
            &selected(&[&headers[1], "col2"]),
        )
        .expect("wide input HTML should render");

        assert!(report.contains("... 3 more"));
        assert!(report.contains("path: ppppp"));
        assert!(report.contains("...</span>"));
        assert!(report.contains("selected_xxxxxxxxx"));
        assert!(!report.contains(&"p".repeat(180)));
        assert!(!report.contains(&headers[1]));
    }

    #[test]
    fn input_data_readiness_html_bounds_tensor_preview_values() {
        let csv = "\
sales,cost
1,2
3,4
5,6
7,8
9,10
11,12
13,14
";
        let table = Table::from_csv_str(csv).expect("long numeric fixture CSV should parse");
        let report = render_input_data_readiness_html_report(
            "path: long.csv",
            &table,
            &selected(&["sales", "cost"]),
        )
        .expect("long numeric input HTML should render");

        assert!(report.contains(
            "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, ... 2 more]"
        ));
        assert!(!report.contains(
            "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0]"
        ));
    }

    #[test]
    fn missing_value_report_matches_expected_markdown() {
        let report = render_fixture_report("fixture: missing.csv", MISSING_CSV, &["sales", "cost"]);

        assert_eq!(
            report,
            "\
# matten data-readiness report

## Input
fixture: missing.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 1 |

## Numeric conversion
strict conversion: error: missing value in column \"cost\", CSV line 3
"
        );
    }

    #[test]
    fn non_numeric_report_matches_expected_markdown() {
        let report = render_fixture_report(
            "fixture: non_numeric.csv",
            NON_NUMERIC_CSV,
            &["sales", "cost"],
        );

        assert_eq!(
            report,
            "\
# matten data-readiness report

## Input
fixture: non_numeric.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: error: non-numeric value \"oops\" in column \"sales\", CSV line 3
"
        );
    }

    #[test]
    fn selected_column_errors_are_readable() {
        let table = Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");

        let missing = render_table_report("fixture: small.csv", &table, &selected(&["profit"]))
            .unwrap_err()
            .to_string();
        assert!(missing.contains("column \"profit\" does not exist"));

        let duplicate =
            render_table_report("fixture: small.csv", &table, &selected(&["sales", "sales"]))
                .unwrap_err()
                .to_string();
        assert!(duplicate.contains("column \"sales\" was selected more than once"));
    }
}
