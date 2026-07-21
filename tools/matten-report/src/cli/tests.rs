use super::*;

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn selected(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

mod format_policy;

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
    assert!(matches!(config.input, Input::Demo));
    assert_eq!(config.kind, ReportKind::ShapeFlow);
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
    assert!(matches!(config.input, Input::Demo));
    assert_eq!(config.kind, ReportKind::DynamicReadiness);
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
    assert!(matches!(config.input, Input::Demo));
    assert_eq!(config.kind, ReportKind::MlprepStandardization);
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
    assert!(matches!(config.input, Input::Demo));
    assert_eq!(config.kind, ReportKind::EducationalPath);
    assert!(config.select.is_empty());
    assert_eq!(
        config.output,
        Some(PathBuf::from("target/matten-report-educational-path.md"))
    );
    assert_eq!(config.format, OutputFormat::Markdown);
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

    assert!(err.contains("unsupported --kind \"dynamic-readiness\"; expected \"data-readiness\""));
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

    assert!(
        err.contains("unsupported --kind \"mlprep-standardization\"; expected \"data-readiness\"")
    );
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

    assert!(err.contains("unsupported --kind \"educational-path\"; expected \"data-readiness\""));
}

#[test]
fn educational_path_demo_rejects_select() {
    let err = parse_args(args(&["--demo", "educational-path", "--select", "sales"])).unwrap_err();

    assert!(err.contains("--select is only accepted with --input"));
}

#[test]
fn unsupported_demo_label_remains_readable() {
    let err = parse_args(args(&["--demo", "unknown"])).unwrap_err();

    assert!(err.contains("unsupported --demo \"unknown\"; expected \"data-readiness\", \"shape-flow\", \"dynamic-readiness\", \"mlprep-standardization\", or \"educational-path\""));
}
