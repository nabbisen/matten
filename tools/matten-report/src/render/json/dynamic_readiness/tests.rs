use super::render;

fn dynamic_readiness_data() -> crate::report::dynamic_readiness::DynamicReadinessReportData {
    crate::report::dynamic_readiness::build().expect("dynamic-readiness data should build")
}

#[test]
fn dynamic_readiness_json_report_matches_expected_snapshot() {
    let data = dynamic_readiness_data();
    let report = render(&data).expect("dynamic-readiness JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "dynamic-readiness",
  "input_mode": "demo",
  "data": {
    "shape": [
      2,
      3
    ],
    "values": [
      {
        "row": 0,
        "column": 0,
        "element": "Float(1.0)"
      },
      {
        "row": 0,
        "column": 1,
        "element": "Text(\"2.5\")"
      },
      {
        "row": 0,
        "column": 2,
        "element": "None"
      },
      {
        "row": 1,
        "column": 0,
        "element": "Int(4)"
      },
      {
        "row": 1,
        "column": 1,
        "element": "Text(\"6.0\")"
      },
      {
        "row": 1,
        "column": 2,
        "element": "Float(8.0)"
      }
    ],
    "schema_summary": [
      {
        "label": "Float",
        "count": 2
      },
      {
        "label": "Int",
        "count": 1
      },
      {
        "label": "Text",
        "count": 2
      },
      {
        "label": "None",
        "count": 1
      }
    ],
    "readiness_masks": {
      "none_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          0.0,
          0.0,
          1.0,
          0.0,
          0.0,
          0.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "numeric_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          1.0,
          0.0,
          0.0,
          1.0,
          0.0,
          1.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "strict_numeric_ready": false
    },
    "strict_conversion": {
      "status": "error",
      "message": "error: strict conversion rejects Text and None values"
    },
    "explicit_policy_conversion": {
      "policy": "none_as(0.0) + allow_text_parse()",
      "tensor": {
        "shape": [
          2,
          3
        ],
        "values": [
          1.0,
          2.5,
          0.0,
          4.0,
          6.0,
          8.0
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
