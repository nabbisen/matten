use super::*;

#[test]
fn data_readiness_json_report_matches_expected_snapshot() {
    let data =
        crate::report::data_readiness::build_demo().expect("demo data-readiness data should build");
    let report =
        render_data_readiness_json_report(&data).expect("data-readiness JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "data-readiness",
  "input_mode": "demo",
  "data": {
    "input_label": "demo: data-readiness",
    "source_columns": [
      "region",
      "sales",
      "cost",
      "note"
    ],
    "selected_columns": [
      "sales",
      "cost"
    ],
    "left_out_columns": [
      "region",
      "note"
    ],
    "missing_counts": [
      {
        "column": "sales",
        "missing": 0
      },
      {
        "column": "cost",
        "missing": 0
      }
    ],
    "numeric_conversion": {
      "status": "success",
      "tensor": {
        "shape": [
          3,
          2
        ],
        "values": [
          100.0,
          40.0,
          150.0,
          45.0,
          120.0,
          55.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      }
    }
  }
}
"#
    );
}
