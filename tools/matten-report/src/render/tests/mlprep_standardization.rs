use super::*;

mod html;

fn mlprep_standardization_data()
-> crate::report::mlprep_standardization::MlprepStandardizationReportData {
    crate::report::mlprep_standardization::build()
        .expect("mlprep-standardization data should build")
}

#[test]
fn mlprep_standardization_json_report_matches_expected_snapshot() {
    let data = mlprep_standardization_data();
    let report = render_mlprep_standardization_json_report(&data)
        .expect("mlprep-standardization JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "mlprep-standardization",
  "input_mode": "demo",
  "data": {
    "selected_columns": [
      "feature_0",
      "feature_1"
    ],
    "operation": "standardize_columns(input)",
    "before": {
      "tensor": {
        "shape": [
          3,
          2
        ],
        "values": [
          8.0,
          80.0,
          10.0,
          100.0,
          12.0,
          120.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "column_mean": [
        10.0,
        100.0
      ],
      "column_population_std": [
        1.632993161855452,
        16.32993161855452
      ]
    },
    "after": {
      "tensor": {
        "shape": [
          3,
          2
        ],
        "values": [
          -1.224744871391589,
          -1.224744871391589,
          0.0,
          0.0,
          1.224744871391589,
          1.224744871391589
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "column_mean": [
        0.0,
        0.0
      ],
      "column_population_std": [
        0.9999999999999999,
        0.9999999999999999
      ]
    }
  }
}
"#
    );
}

#[test]
fn mlprep_standardization_report_matches_expected_markdown() {
    let data = mlprep_standardization_data();
    let report = render_mlprep_standardization_report(&data)
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
