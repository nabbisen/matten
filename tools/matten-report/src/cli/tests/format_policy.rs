use super::*;

#[test]
fn educational_path_html_requires_output() {
    let err = parse_args(args(&["--demo", "educational-path", "--format", "html"])).unwrap_err();

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
    assert!(matches!(config.input, Input::Demo));
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
    assert!(matches!(config.input, Input::Demo));
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
    assert!(matches!(config.input, Input::Demo));
    assert_eq!(config.format, OutputFormat::Html);
    assert_eq!(
        config.output,
        Some(PathBuf::from("target/matten-report-shape-flow.html"))
    );
}

#[test]
fn dynamic_readiness_html_requires_output() {
    let err = parse_args(args(&["--demo", "dynamic-readiness", "--format", "html"])).unwrap_err();

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
    assert!(matches!(config.input, Input::Demo));
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
    assert!(matches!(config.input, Input::Demo));
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
    assert_eq!(config.kind, ReportKind::DataReadiness);
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

    assert!(
        err.contains("unsupported --format \"svg\"; expected \"markdown\", \"html\", or \"json\"")
    );
}

#[test]
fn fixed_demo_json_requires_output() {
    let err = parse_args(args(&["--demo", "shape-flow", "--format", "json"])).unwrap_err();

    assert!(err.contains("--format json requires --output <report.json>"));
}

#[test]
fn fixed_demo_json_allows_explicit_output() {
    let action = parse_args(args(&[
        "--demo",
        "shape-flow",
        "--format",
        "json",
        "--output",
        "target/matten-report-shape-flow.json",
    ]))
    .expect("shape-flow JSON with output should parse");

    let Action::Run(config) = action else {
        panic!("expected run action");
    };
    assert!(matches!(config.input, Input::Demo));
    assert_eq!(config.format, OutputFormat::Json);
    assert_eq!(
        config.output,
        Some(PathBuf::from("target/matten-report-shape-flow.json"))
    );
}
