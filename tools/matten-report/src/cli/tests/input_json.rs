use super::*;

#[test]
fn input_mode_json_requires_output() {
    let err = parse_args(args(&[
        "--input",
        "fixtures/small.csv",
        "--kind",
        "data-readiness",
        "--select",
        "sales,cost",
        "--format",
        "json",
    ]))
    .unwrap_err();

    assert!(err.contains("--format json requires --output <report.json>"));
}

#[test]
fn input_mode_json_allows_explicit_output() {
    let action = parse_args(args(&[
        "--input",
        "fixtures/small.csv",
        "--kind",
        "data-readiness",
        "--select",
        "sales,cost",
        "--format",
        "json",
        "--output",
        "target/matten-report-input.json",
    ]))
    .expect("input-mode data-readiness JSON with output should parse");

    let Action::Run(config) = action else {
        panic!("expected run action");
    };
    let Input::CsvPath { path } = config.input else {
        panic!("expected CSV input");
    };
    assert_eq!(path, PathBuf::from("fixtures/small.csv"));
    assert_eq!(config.kind, ReportKind::DataReadiness);
    assert_eq!(config.select, selected(&["sales", "cost"]));
    assert_eq!(config.format, OutputFormat::Json);
    assert_eq!(
        config.output,
        Some(PathBuf::from("target/matten-report-input.json"))
    );
}
