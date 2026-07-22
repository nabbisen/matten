use super::*;

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
