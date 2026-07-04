use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use matten_data::{MattenDataError, Table};

const DEMO_CSV: &str = "\
region,sales,cost,note
north,100,40,ok
south,150,45,review
east,120,55,ok";

const KIND_DATA_READINESS: &str = "data-readiness";

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
                    "--select is only accepted with --input; demo mode uses a fixed selection\n\n{}",
                    usage()
                ));
            }
            Ok(Action::Run(Config {
                input: Input::Demo {
                    label: label.clone(),
                },
                kind: label,
                select: vec!["sales".to_string(), "cost".to_string()],
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
    if label != KIND_DATA_READINESS {
        return Err(format!(
            "unsupported --demo {label:?}; expected {KIND_DATA_READINESS:?}"
        ));
    }
    if let Some(kind) = kind {
        if kind != KIND_DATA_READINESS {
            return Err(format!(
                "unsupported --kind {kind:?}; expected {KIND_DATA_READINESS:?}"
            ));
        }
    }
    Ok(())
}

fn render_report(config: &Config) -> Result<String, Box<dyn Error>> {
    let (input_label, table) = match &config.input {
        Input::Demo { label } => (
            format!("demo: {label}"),
            Table::from_csv_str(DEMO_CSV).map_err(Box::<dyn Error>::from)?,
        ),
        Input::CsvPath { path } => (
            format!("path: {}", path.display()),
            Table::from_csv_path(path).map_err(Box::<dyn Error>::from)?,
        ),
    };

    if config.kind != KIND_DATA_READINESS {
        return Err(format!(
            "unsupported report kind {:?}; expected {KIND_DATA_READINESS:?}",
            config.kind
        )
        .into());
    }

    render_table_report(&input_label, &table, &config.select)
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
  matten-report --input <csv-path> --kind data-readiness --select <col1,col2> [--output <report.md>]

Only the data-readiness report is supported in this first local-tool slice."
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
