use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use matten::Tensor;
use matten_data::{MattenDataError, Table};

const DEMO_CSV: &str = "\
region,sales,cost,note
north,100,40,ok
south,150,45,review
east,120,55,ok";

const KIND_DATA_READINESS: &str = "data-readiness";
const KIND_SHAPE_FLOW: &str = "shape-flow";

#[derive(Debug)]
struct Config {
    input: Input,
    kind: String,
    select: Vec<String>,
    output: Option<PathBuf>,
}

#[derive(Debug)]
enum Input {
    Demo { label: String },
    CsvPath { path: PathBuf },
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
            "--help" | "-h" => return Ok(Action::Help),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }

    match (demo, input) {
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
            }))
        }
        (Some(_), Some(_)) => Err(format!(
            "--demo and --input are mutually exclusive\n\n{}",
            usage()
        )),
        (None, None) => Err(usage()),
    }
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

fn require_kind_or_demo_label(label: &str, kind: Option<&str>) -> Result<(), String> {
    if !is_supported_demo(label) {
        return Err(format!(
            "unsupported --demo {label:?}; expected {KIND_DATA_READINESS:?} or {KIND_SHAPE_FLOW:?}"
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
    matches!(label, KIND_DATA_READINESS | KIND_SHAPE_FLOW)
}

fn render_report(config: &Config) -> Result<String, Box<dyn Error>> {
    match &config.input {
        Input::Demo { label } if label == KIND_SHAPE_FLOW => render_shape_flow_report(),
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
  matten-report --input <csv-path> --kind data-readiness --select <col1,col2> [--output <report.md>]

Demo reports are fixed examples. Input mode supports only data-readiness."
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
    fn unsupported_demo_label_remains_readable() {
        let err = parse_args(args(&["--demo", "unknown"])).unwrap_err();

        assert!(err.contains(
            "unsupported --demo \"unknown\"; expected \"data-readiness\" or \"shape-flow\""
        ));
    }

    #[test]
    fn shape_flow_report_matches_expected_markdown() {
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
    fn success_report_matches_expected_markdown() {
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
