use super::*;

#[test]
fn bounded_strings_use_unicode_scalar_counts_without_ellipsis() {
    let exact = "é".repeat(MAX_DISPLAY_CHARS);
    let exact = bounded_string(&exact, MAX_DISPLAY_CHARS);
    assert!(!exact.truncated);
    assert_eq!(exact.shown_chars, 120);
    assert_eq!(exact.total_chars, 120);

    let long = format!("{}終", "é".repeat(MAX_DISPLAY_CHARS));
    let bounded = bounded_string(&long, MAX_DISPLAY_CHARS);
    assert!(bounded.truncated);
    assert_eq!(bounded.shown_chars, 120);
    assert_eq!(bounded.total_chars, 121);
    assert_eq!(bounded.value, "é".repeat(120));
    assert!(!bounded.value.contains("..."));
}

#[test]
fn bounded_lists_keep_first_twelve_and_report_totals() {
    let values: Vec<String> = (0..14).map(|index| format!("column-{index}")).collect();
    let bounded = bounded_strings(&values);

    assert!(bounded.truncated);
    assert_eq!(bounded.shown_items, 12);
    assert_eq!(bounded.total_items, 14);
    assert_eq!(bounded.limit, 12);
    assert_eq!(bounded.items[0].value, "column-0");
    assert_eq!(bounded.items[11].value, "column-11");
}

#[test]
fn conversion_errors_use_the_240_character_limit() {
    let exact = bounded_string(&"e".repeat(MAX_ERROR_CHARS), MAX_ERROR_CHARS);
    assert!(!exact.truncated);
    assert_eq!(exact.shown_chars, 240);

    let long = bounded_string(&"e".repeat(MAX_ERROR_CHARS + 1), MAX_ERROR_CHARS);
    assert!(long.truncated);
    assert_eq!(long.shown_chars, 240);
    assert_eq!(long.total_chars, 241);
}

#[test]
fn json_encoding_round_trips_user_controlled_text() {
    let hostile = "quote\" slash\\ control\n雪";
    let data = DataReadinessReportData {
        input_label: format!("path: {hostile}"),
        source_columns: vec![hostile.to_string()],
        selected_columns: vec![hostile.to_string()],
        left_out_columns: Vec::new(),
        missing_counts: vec![DataReadinessMissingCount {
            column: hostile.to_string(),
            missing: 0,
        }],
        conversion: DataReadinessConversion::Error {
            message: hostile.to_string(),
        },
    };
    let report = render(&data).expect("hostile text should encode as JSON");
    let value: serde_json::Value = serde_json::from_str(&report).expect("JSON should parse");

    assert_eq!(
        value["data"]["input_label"]["value"],
        format!("path: {hostile}")
    );
    assert_eq!(
        value["data"]["source_columns"]["items"][0]["value"],
        hostile
    );
    assert_eq!(
        value["data"]["numeric_conversion"]["message"]["value"],
        hostile
    );
}

#[test]
fn rendered_lists_and_nested_names_are_bounded() {
    let long_name = "x".repeat(MAX_DISPLAY_CHARS + 1);
    let columns: Vec<String> = (0..14)
        .map(|index| {
            if index == 0 {
                long_name.clone()
            } else {
                format!("column-{index}")
            }
        })
        .collect();
    let data = DataReadinessReportData {
        input_label: format!("path: {}", "p".repeat(MAX_DISPLAY_CHARS + 1)),
        source_columns: columns.clone(),
        selected_columns: columns.clone(),
        left_out_columns: columns.clone(),
        missing_counts: columns
            .iter()
            .map(|column| DataReadinessMissingCount {
                column: column.clone(),
                missing: 0,
            })
            .collect(),
        conversion: DataReadinessConversion::Error {
            message: "e".repeat(MAX_ERROR_CHARS + 1),
        },
    };
    let report = render(&data).expect("wide input should render as bounded JSON");
    let value: serde_json::Value = serde_json::from_str(&report).expect("JSON should parse");

    for name in [
        "source_columns",
        "selected_columns",
        "left_out_columns",
        "missing_counts",
    ] {
        let list = &value["data"][name];
        assert_eq!(list["shown_items"], 12);
        assert_eq!(list["total_items"], 14);
        assert_eq!(list["truncated"], true);
    }
    assert_eq!(
        value["data"]["source_columns"]["items"][0]["shown_chars"],
        120
    );
    assert_eq!(
        value["data"]["missing_counts"]["items"][0]["column"]["shown_chars"],
        120
    );
    assert_eq!(
        value["data"]["numeric_conversion"]["message"]["shown_chars"],
        240
    );
    assert!(!report.contains("column-13"));
    assert!(!report.contains("... 2 more"));
    assert!(!report.contains(&long_name));
}

#[test]
fn input_tensor_preview_reuses_bounds_and_non_finite_rejection() {
    let values: Vec<f64> = (1..=14).map(f64::from).collect();
    let data = DataReadinessReportData {
        input_label: "path: long.csv".to_string(),
        source_columns: vec!["sales".to_string(), "cost".to_string()],
        selected_columns: vec!["sales".to_string(), "cost".to_string()],
        left_out_columns: Vec::new(),
        missing_counts: Vec::new(),
        conversion: DataReadinessConversion::Success {
            tensor_shape: vec![7, 2],
            tensor_values: values,
        },
    };
    let report = render(&data).expect("finite input JSON should render");
    let value: serde_json::Value = serde_json::from_str(&report).expect("JSON should parse");
    let tensor = &value["data"]["numeric_conversion"]["tensor"];
    assert_eq!(tensor["shown_values"], 12);
    assert_eq!(tensor["total_values"], 14);
    assert_eq!(tensor["truncated"], true);

    for non_finite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let data = DataReadinessReportData {
            input_label: "path: non-finite.csv".to_string(),
            source_columns: vec!["value".to_string()],
            selected_columns: vec!["value".to_string()],
            left_out_columns: Vec::new(),
            missing_counts: Vec::new(),
            conversion: DataReadinessConversion::Success {
                tensor_shape: vec![1, 1],
                tensor_values: vec![non_finite],
            },
        };
        let error = render(&data).expect_err("non-finite input JSON must fail");
        assert_eq!(
            error.to_string(),
            "JSON report encountered a non-finite numeric value"
        );
    }
}
