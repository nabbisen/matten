use std::path::PathBuf;

use crate::request::{
    Config, Input, KIND_DATA_READINESS, KIND_DYNAMIC_READINESS, KIND_EDUCATIONAL_PATH,
    KIND_MLPREP_STANDARDIZATION, KIND_SHAPE_FLOW, OutputFormat, ReportKind, SUPPORTED_DEMOS,
};

#[derive(Debug)]
pub(crate) enum Action {
    Run(Config),
    Help,
}

pub(crate) fn parse_args<I>(args: I) -> Result<Action, String>
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
            "--demo" => demo = Some(take_value(&mut args, "--demo")?),
            "--input" => input = Some(PathBuf::from(take_value(&mut args, "--input")?)),
            "--kind" => kind = Some(take_value(&mut args, "--kind")?),
            "--select" => select = Some(parse_select(&take_value(&mut args, "--select")?)?),
            "--output" => output = Some(PathBuf::from(take_value(&mut args, "--output")?)),
            "--format" => format = parse_format(&take_value(&mut args, "--format")?)?,
            "--help" | "-h" => return Ok(Action::Help),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }

    let action = match (demo, input) {
        (Some(label), None) => {
            let report_kind = require_kind_or_demo_label(&label, kind.as_deref())?;
            if select.is_some() {
                return Err(format!(
                    "--select is only accepted with --input; demo mode uses fixed inputs\n\n{}",
                    usage()
                ));
            }
            let select = if report_kind == ReportKind::DataReadiness {
                vec!["sales".to_string(), "cost".to_string()]
            } else {
                Vec::new()
            };
            Ok(Action::Run(Config {
                input: Input::Demo,
                kind: report_kind,
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
                kind: ReportKind::DataReadiness,
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
        "json" => Ok(OutputFormat::Json),
        other => Err(format!(
            "unsupported --format {other:?}; expected \"markdown\", \"html\", or \"json\""
        )),
    }
}

fn validate_format_policy(config: &Config) -> Result<(), String> {
    match config.format {
        OutputFormat::Markdown => Ok(()),
        OutputFormat::Html => validate_html_format_policy(config),
        OutputFormat::Json => validate_json_format_policy(config),
    }
}

fn validate_html_format_policy(config: &Config) -> Result<(), String> {
    if config.output.is_none() {
        return Err("--format html requires --output <report.html>".to_string());
    }
    match config.input {
        Input::Demo if supports_html_demo(config.kind) => Ok(()),
        Input::Demo => Err(format!(
            "--format html is only supported for --demo {}; got {:?}",
            supported_html_demos(),
            config.kind.as_str()
        )),
        Input::CsvPath { .. } if config.kind == ReportKind::DataReadiness => Ok(()),
        Input::CsvPath { .. } => Err(format!(
            "--format html is only supported for --input <csv-path> --kind {KIND_DATA_READINESS}"
        )),
    }
}

fn validate_json_format_policy(config: &Config) -> Result<(), String> {
    if config.output.is_none() {
        return Err("--format json requires --output <report.json>".to_string());
    }
    match config.input {
        Input::Demo if supports_json_demo(config.kind) => Ok(()),
        Input::Demo => Err(format!(
            "--format json is only supported for --demo {}; got {:?}",
            supported_json_demos(),
            config.kind.as_str()
        )),
        Input::CsvPath { .. } => Err("--format json is not supported for --input yet".to_string()),
    }
}

fn supports_html_demo(_kind: ReportKind) -> bool {
    true
}

fn supported_html_demos() -> &'static str {
    SUPPORTED_DEMOS
}

fn supports_json_demo(kind: ReportKind) -> bool {
    supports_html_demo(kind)
}

fn supported_json_demos() -> &'static str {
    supported_html_demos()
}

fn require_kind_or_demo_label(label: &str, kind: Option<&str>) -> Result<ReportKind, String> {
    let report_kind = ReportKind::from_label(label).ok_or_else(|| {
        format!(
            "unsupported --demo {label:?}; expected {KIND_DATA_READINESS:?}, {KIND_SHAPE_FLOW:?}, {KIND_DYNAMIC_READINESS:?}, {KIND_MLPREP_STANDARDIZATION:?}, or {KIND_EDUCATIONAL_PATH:?}"
        )
    })?;
    if let Some(kind) = kind
        && kind != label
    {
        return Err(format!(
            "unsupported --kind {kind:?} for --demo {label:?}; expected {label:?}"
        ));
    }
    Ok(report_kind)
}

pub(crate) fn usage() -> String {
    "\
Usage:
  matten-report --demo data-readiness [--output <report.md>]
  matten-report --demo data-readiness --format html --output <report.html>
  matten-report --demo data-readiness --format json --output <report.json>
  matten-report --demo shape-flow [--output <report.md>]
  matten-report --demo shape-flow --format html --output <report.html>
  matten-report --demo shape-flow --format json --output <report.json>
  matten-report --demo dynamic-readiness [--output <report.md>]
  matten-report --demo dynamic-readiness --format html --output <report.html>
  matten-report --demo dynamic-readiness --format json --output <report.json>
  matten-report --demo mlprep-standardization [--output <report.md>]
  matten-report --demo mlprep-standardization --format html --output <report.html>
  matten-report --demo mlprep-standardization --format json --output <report.json>
  matten-report --demo educational-path [--format markdown] [--output <report.md>]
  matten-report --demo educational-path --format html --output <report.html>
  matten-report --demo educational-path --format json --output <report.json>
  matten-report --input <csv-path> --kind data-readiness --select <col1,col2> [--output <report.md>]
  matten-report --input <csv-path> --kind data-readiness --select <col1,col2> --format html --output <report.html>

Demo reports are fixed examples. Input mode supports only data-readiness.
Markdown is the default format. HTML and private fixed-demo JSON are local file output and require --output."
        .to_string()
}

#[cfg(test)]
mod tests;
