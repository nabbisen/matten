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
        Input::Demo { label } if label == KIND_EDUCATIONAL_PATH => Ok(()),
        Input::Demo { label } => Err(format!(
            "--format html is only supported for --demo {KIND_EDUCATIONAL_PATH:?}; got {label:?}"
        )),
        Input::CsvPath { .. } => Err(format!(
            "--format html is only supported for --demo {KIND_EDUCATIONAL_PATH:?}"
        )),
    }
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
            Input::Demo { label } if label == KIND_EDUCATIONAL_PATH => {
                render_educational_path_html_report()
            }
            Input::Demo { label } => Err(format!(
                "--format html is only supported for --demo {KIND_EDUCATIONAL_PATH:?}; got {label:?}"
            )
            .into()),
            Input::CsvPath { .. } => Err(format!(
                "--format html is only supported for --demo {KIND_EDUCATIONAL_PATH:?}"
            )
            .into()),
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

fn render_shape_flow_report() -> Result<String, Box<dyn Error>> {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);
    let broadcast = &a + &b;
    let reshaped = a.reshape(&[3, 2]);
    let mean_axis_0 = a.mean_axis(0);
    let mean_axis_1 = a.mean_axis(1);
    let left = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let right = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    let product = left.matmul(&right);

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
    writeln!(report, "input a: shape {:?}", a.shape())?;
    writeln!(report, "input b: shape {:?}", b.shape())?;
    writeln!(report, "operation: a + b")?;
    writeln!(
        report,
        "shape flow: {:?} + {:?} -> {:?}",
        a.shape(),
        b.shape(),
        broadcast.shape()
    )?;
    writeln!(report, "result values: {:?}", broadcast.as_slice())?;
    writeln!(report)?;

    writeln!(report, "## Reshape")?;
    writeln!(report, "input: shape {:?}", a.shape())?;
    writeln!(report, "operation: reshape([3, 2])")?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        a.shape(),
        reshaped.shape()
    )?;
    writeln!(report, "result values: {:?}", reshaped.as_slice())?;
    writeln!(report)?;

    writeln!(report, "## Axis reductions")?;
    writeln!(report, "input: shape {:?}", a.shape())?;
    writeln!(
        report,
        "mean_axis(0): {:?} -> {:?}",
        a.shape(),
        mean_axis_0.shape()
    )?;
    writeln!(report, "mean_axis(0) values: {:?}", mean_axis_0.as_slice())?;
    writeln!(
        report,
        "mean_axis(1): {:?} -> {:?}",
        a.shape(),
        mean_axis_1.shape()
    )?;
    writeln!(report, "mean_axis(1) values: {:?}", mean_axis_1.as_slice())?;
    writeln!(report)?;

    writeln!(report, "## Matrix multiplication")?;
    writeln!(report, "left: shape {:?}", left.shape())?;
    writeln!(report, "right: shape {:?}", right.shape())?;
    writeln!(report, "operation: left.matmul(right)")?;
    writeln!(
        report,
        "shape flow: {:?} @ {:?} -> {:?}",
        left.shape(),
        right.shape(),
        product.shape()
    )?;
    writeln!(report, "result values: {:?}", product.as_slice())?;

    Ok(report)
}

fn render_dynamic_readiness_report() -> Result<String, Box<dyn Error>> {
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
    writeln!(report, "shape: {:?}", dynamic.shape())?;
    writeln!(report, "row-major values:")?;
    write_dynamic_values(&mut report, &dynamic)?;
    writeln!(report, "schema summary:")?;
    write_dynamic_schema_summary(&mut report, &dynamic)?;
    writeln!(report)?;

    writeln!(report, "## Readiness masks")?;
    writeln!(report, "none mask: {:?}", none_mask.as_slice())?;
    writeln!(
        report,
        "numeric mask: strict policy readiness {:?}",
        numeric_mask.as_slice()
    )?;
    writeln!(
        report,
        "strict numeric-ready: {}",
        dynamic.is_numeric_convertible()
    )?;
    writeln!(report)?;

    writeln!(report, "## Strict conversion")?;
    if dynamic.try_numeric().is_err() {
        writeln!(
            report,
            "result: error: strict conversion rejects Text and None values"
        )?;
    } else {
        return Err("strict dynamic conversion unexpectedly succeeded".into());
    }
    writeln!(report)?;

    writeln!(report, "## Explicit policy conversion")?;
    writeln!(report, "policy: none_as(0.0) + allow_text_parse()")?;
    writeln!(report, "converted shape: {:?}", converted.shape())?;
    writeln!(
        report,
        "converted row-major values: {:?}",
        converted.as_slice()
    )?;

    Ok(report)
}

fn render_mlprep_standardization_report() -> Result<String, Box<dyn Error>> {
    let input = Tensor::new(vec![8.0, 80.0, 10.0, 100.0, 12.0, 120.0], &[3, 2]);
    let standardized = standardize_columns(&input).map_err(Box::<dyn Error>::from)?;
    let before_mean = input.mean_axis(0);
    let before_std = input.std_axis(0);
    let after_mean = standardized.mean_axis(0);
    let after_std = standardized.std_axis(0);

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
    writeln!(report, "shape: {:?}", input.shape())?;
    writeln!(
        report,
        "row-major values: {}",
        format_fixed_values(input.as_slice())
    )?;
    writeln!(
        report,
        "column mean: {}",
        format_fixed_values(before_mean.as_slice())
    )?;
    writeln!(
        report,
        "column population std: {}",
        format_fixed_values(before_std.as_slice())
    )?;
    writeln!(report)?;

    writeln!(report, "## After")?;
    writeln!(report, "shape: {:?}", standardized.shape())?;
    writeln!(
        report,
        "row-major values: {}",
        format_fixed_values(standardized.as_slice())
    )?;
    writeln!(
        report,
        "column mean: {}",
        format_fixed_values(after_mean.as_slice())
    )?;
    writeln!(
        report,
        "column population std: {}",
        format_fixed_values(after_std.as_slice())
    )?;
    writeln!(report)?;

    writeln!(report, "## Shape meaning")?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        input.shape(),
        standardized.shape()
    )?;
    writeln!(report, "rows: samples unchanged")?;
    writeln!(report, "columns: features unchanged")?;

    Ok(report)
}

fn render_educational_path_report() -> Result<String, Box<dyn Error>> {
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
    writeln!(report, "1. ask what shape each input has")?;
    writeln!(report, "2. ask which axes align, disappear, or remain")?;
    writeln!(report, "3. read the output shape before reading values")?;
    writeln!(report, "4. convert dynamic data before numeric computation")?;
    writeln!(report)?;

    writeln!(report, "## Broadcasting")?;
    writeln!(
        report,
        "shape flow: {:?} + {:?} -> {:?}",
        broadcast_left.shape(),
        broadcast_right.shape(),
        broadcast.shape()
    )?;
    writeln!(report, "axis 1: left repeats across 4 columns")?;
    writeln!(report, "axis 0: right repeats across 3 rows")?;
    writeln!(report, "result values: {:?}", broadcast.as_slice())?;
    writeln!(report)?;

    writeln!(report, "## Reshape and transpose")?;
    writeln!(
        report,
        "reshape: {:?} -> {:?}",
        shape_input.shape(),
        reshaped.shape()
    )?;
    writeln!(report, "reshape values: {:?}", reshaped.as_slice())?;
    writeln!(
        report,
        "transpose: {:?} -> {:?}",
        shape_input.shape(),
        transposed.shape()
    )?;
    writeln!(report, "transpose values: {:?}", transposed.as_slice())?;
    writeln!(
        report,
        "meaning: reshape changes grouping; transpose changes coordinate meaning"
    )?;
    writeln!(report)?;

    writeln!(report, "## Axis reductions")?;
    writeln!(
        report,
        "mean_axis(0): {:?} -> {:?}",
        shape_input.shape(),
        mean_axis_0.shape()
    )?;
    writeln!(
        report,
        "mean_axis(0) keeps columns: {:?}",
        mean_axis_0.as_slice()
    )?;
    writeln!(
        report,
        "mean_axis(1): {:?} -> {:?}",
        shape_input.shape(),
        mean_axis_1.shape()
    )?;
    writeln!(
        report,
        "mean_axis(1) keeps rows: {:?}",
        mean_axis_1.as_slice()
    )?;
    writeln!(report)?;

    writeln!(report, "## Matrix multiplication")?;
    writeln!(
        report,
        "shape flow: {:?} @ {:?} -> {:?}",
        matmul_left.shape(),
        matmul_right.shape(),
        matmul.shape()
    )?;
    writeln!(report, "shared inner dimension: 3")?;
    writeln!(report, "result values: {:?}", matmul.as_slice())?;
    writeln!(report)?;

    writeln!(report, "## Dynamic readiness")?;
    writeln!(report, "dynamic shape: {:?}", dynamic.shape())?;
    writeln!(report, "none mask: {:?}", none_mask.as_slice())?;
    writeln!(
        report,
        "numeric mask: strict policy readiness {:?}",
        numeric_mask.as_slice()
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
        standardization_input.shape(),
        standardized.shape()
    )?;
    writeln!(
        report,
        "before column mean: {}",
        format_fixed_values(before_mean.as_slice())
    )?;
    writeln!(
        report,
        "before column population std: {}",
        format_fixed_values(before_std.as_slice())
    )?;
    writeln!(
        report,
        "after column mean: {}",
        format_fixed_values(after_mean.as_slice())
    )?;
    writeln!(
        report,
        "after column population std: {}",
        format_fixed_values(after_std.as_slice())
    )?;
    writeln!(report)?;

    writeln!(report, "## What this report is not")?;
    writeln!(report, "- not a public API")?;
    writeln!(report, "- not source scanning")?;
    writeln!(report, "- not a renderer")?;
    writeln!(report, "- not model-quality analysis")?;

    Ok(report)
}

fn render_educational_path_html_report() -> Result<String, Box<dyn Error>> {
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

    let mut report = String::new();
    writeln!(report, "<!doctype html>")?;
    writeln!(report, "<html lang=\"en\">")?;
    writeln!(report, "<head>")?;
    writeln!(report, "  <meta charset=\"utf-8\">")?;
    writeln!(
        report,
        "  <title>{}</title>",
        html_escape("matten educational-path report")
    )?;
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
    writeln!(
        report,
        "<h1>{}</h1>",
        html_escape("matten educational-path report")
    )?;
    writeln!(
        report,
        "<p class=\"note\">{}</p>",
        html_escape("Fixed educational demo report, not automatic expression tracing.")
    )?;

    writeln!(report, "<section>")?;
    writeln!(
        report,
        "<h2>{}</h2>",
        html_escape("How to read shapes first")
    )?;
    writeln!(report, "<ol>")?;
    for item in [
        "ask what shape each input has",
        "ask which axes align, disappear, or remain",
        "read the output shape before reading values",
        "convert dynamic data before numeric computation",
    ] {
        writeln!(report, "<li>{}</li>", html_escape(item))?;
    }
    writeln!(report, "</ol>")?;
    writeln!(report, "</section>")?;

    writeln!(report, "<section>")?;
    writeln!(report, "<h2>{}</h2>", html_escape("Broadcasting"))?;
    write_shape_flow_table(
        &mut report,
        &[
            ("left", format!("{:?}", broadcast_left.shape())),
            ("right", format!("{:?}", broadcast_right.shape())),
            ("result", format!("{:?}", broadcast.shape())),
        ],
    )?;
    writeln!(
        report,
        "<p>{}</p>",
        html_escape("axis 1: left repeats across 4 columns; axis 0: right repeats across 3 rows")
    )?;
    write_html_pre(
        &mut report,
        &format!("result values: {:?}", broadcast.as_slice()),
    )?;
    writeln!(report, "</section>")?;

    writeln!(report, "<section>")?;
    writeln!(report, "<h2>{}</h2>", html_escape("Reshape and transpose"))?;
    write_shape_flow_table(
        &mut report,
        &[
            ("input", format!("{:?}", shape_input.shape())),
            ("reshape", format!("{:?}", reshaped.shape())),
            ("transpose", format!("{:?}", transposed.shape())),
        ],
    )?;
    write_html_pre(
        &mut report,
        &format!(
            "reshape values: {:?}\ntranspose values: {:?}",
            reshaped.as_slice(),
            transposed.as_slice()
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
        &mut report,
        &[
            (
                "mean_axis(0)",
                format!("{:?} -> {:?}", shape_input.shape(), mean_axis_0.shape()),
            ),
            (
                "mean_axis(1)",
                format!("{:?} -> {:?}", shape_input.shape(), mean_axis_1.shape()),
            ),
        ],
    )?;
    write_html_pre(
        &mut report,
        &format!(
            "mean_axis(0) keeps columns: {:?}\nmean_axis(1) keeps rows: {:?}",
            mean_axis_0.as_slice(),
            mean_axis_1.as_slice()
        ),
    )?;
    writeln!(report, "</section>")?;

    writeln!(report, "<section>")?;
    writeln!(report, "<h2>{}</h2>", html_escape("Matrix multiplication"))?;
    write_shape_flow_table(
        &mut report,
        &[
            ("left", format!("{:?}", matmul_left.shape())),
            ("right", format!("{:?}", matmul_right.shape())),
            ("result", format!("{:?}", matmul.shape())),
        ],
    )?;
    writeln!(
        report,
        "<p>{}</p>",
        html_escape("shared inner dimension: 3")
    )?;
    write_html_pre(
        &mut report,
        &format!("result values: {:?}", matmul.as_slice()),
    )?;
    writeln!(report, "</section>")?;

    writeln!(report, "<section>")?;
    writeln!(report, "<h2>{}</h2>", html_escape("Dynamic readiness"))?;
    write_shape_flow_table(
        &mut report,
        &[
            ("dynamic shape", format!("{:?}", dynamic.shape())),
            ("none mask", format!("{:?}", none_mask.as_slice())),
            (
                "numeric mask",
                format!("strict policy readiness {:?}", numeric_mask.as_slice()),
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
        &mut report,
        &[
            (
                "shape flow",
                format!(
                    "{:?} -> {:?}",
                    standardization_input.shape(),
                    standardized.shape()
                ),
            ),
            ("before mean", format_fixed_values(before_mean.as_slice())),
            (
                "before population std",
                format_fixed_values(before_std.as_slice()),
            ),
            ("after mean", format_fixed_values(after_mean.as_slice())),
            (
                "after population std",
                format_fixed_values(after_std.as_slice()),
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
    for item in [
        "not a public API",
        "not source scanning",
        "not a renderer",
        "not model-quality analysis",
    ] {
        writeln!(report, "<li>{}</li>", html_escape(item))?;
    }
    writeln!(report, "</ul>")?;
    writeln!(report, "</section>")?;

    writeln!(report, "</main>")?;
    writeln!(report, "</body>")?;
    writeln!(report, "</html>")?;

    Ok(report)
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

fn write_dynamic_values(report: &mut String, tensor: &Tensor) -> Result<(), std::fmt::Error> {
    let shape = tensor.shape();
    let columns = shape.get(1).copied().unwrap_or(1);
    for (index, element) in tensor.to_elements().iter().enumerate() {
        let row = index / columns;
        let column = index % columns;
        writeln!(
            report,
            "- [{row}, {column}] {}",
            format_dynamic_element(element)
        )?;
    }
    Ok(())
}

fn write_dynamic_schema_summary(
    report: &mut String,
    tensor: &Tensor,
) -> Result<(), std::fmt::Error> {
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

    writeln!(report, "- Float: {floats}")?;
    writeln!(report, "- Int: {ints}")?;
    writeln!(report, "- Text: {texts}")?;
    if bools > 0 {
        writeln!(report, "- Bool: {bools}")?;
    }
    writeln!(report, "- None: {none}")?;
    Ok(())
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
  matten-report --demo shape-flow [--output <report.md>]
  matten-report --demo dynamic-readiness [--output <report.md>]
  matten-report --demo mlprep-standardization [--output <report.md>]
  matten-report --demo educational-path [--format markdown] [--output <report.md>]
  matten-report --demo educational-path --format html --output <report.html>
  matten-report --input <csv-path> --kind data-readiness --select <col1,col2> [--output <report.md>]

Demo reports are fixed examples. Input mode supports only data-readiness.
Markdown is the default format. HTML is local file output for educational-path only."
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
    fn html_format_is_limited_to_educational_path_demo() {
        let err = parse_args(args(&[
            "--demo",
            "shape-flow",
            "--format",
            "html",
            "--output",
            "target/matten-report-shape-flow.html",
        ]))
        .unwrap_err();

        assert!(err.contains(
            "--format html is only supported for --demo \"educational-path\"; got \"shape-flow\""
        ));
    }

    #[test]
    fn input_mode_does_not_support_html_format() {
        let err = parse_args(args(&[
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
        .unwrap_err();

        assert!(err.contains("--format html is only supported for --demo \"educational-path\""));
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
