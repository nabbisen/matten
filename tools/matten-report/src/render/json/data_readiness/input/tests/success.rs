use super::*;

#[test]
fn input_json_success_is_deterministic_and_structured() {
    let data = build(SMALL_CSV, "path: tools/matten-report/fixtures/small.csv");
    let first = render(&data).expect("input success JSON should render");
    let second = render(&data).expect("input success JSON should render twice");

    assert_eq!(first, second);
    assert!(first.ends_with('\n'));
    assert_eq!(
        first,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "data-readiness",
  "input_mode": "csv",
  "limits": {
    "max_display_columns": 12,
    "max_display_chars": 120,
    "max_error_chars": 240,
    "max_tensor_preview_values": 12
  },
  "data": {
    "input_label": {
      "value": "path: tools/matten-report/fixtures/small.csv",
      "truncated": false,
      "shown_chars": 44,
      "total_chars": 44,
      "limit": 120
    },
    "source_columns": {
      "items": [
        {
          "value": "region",
          "truncated": false,
          "shown_chars": 6,
          "total_chars": 6,
          "limit": 120
        },
        {
          "value": "sales",
          "truncated": false,
          "shown_chars": 5,
          "total_chars": 5,
          "limit": 120
        },
        {
          "value": "cost",
          "truncated": false,
          "shown_chars": 4,
          "total_chars": 4,
          "limit": 120
        },
        {
          "value": "note",
          "truncated": false,
          "shown_chars": 4,
          "total_chars": 4,
          "limit": 120
        }
      ],
      "truncated": false,
      "shown_items": 4,
      "total_items": 4,
      "limit": 12
    },
    "selected_columns": {
      "items": [
        {
          "value": "sales",
          "truncated": false,
          "shown_chars": 5,
          "total_chars": 5,
          "limit": 120
        },
        {
          "value": "cost",
          "truncated": false,
          "shown_chars": 4,
          "total_chars": 4,
          "limit": 120
        }
      ],
      "truncated": false,
      "shown_items": 2,
      "total_items": 2,
      "limit": 12
    },
    "left_out_columns": {
      "items": [
        {
          "value": "region",
          "truncated": false,
          "shown_chars": 6,
          "total_chars": 6,
          "limit": 120
        },
        {
          "value": "note",
          "truncated": false,
          "shown_chars": 4,
          "total_chars": 4,
          "limit": 120
        }
      ],
      "truncated": false,
      "shown_items": 2,
      "total_items": 2,
      "limit": 12
    },
    "missing_counts": {
      "items": [
        {
          "column": {
            "value": "sales",
            "truncated": false,
            "shown_chars": 5,
            "total_chars": 5,
            "limit": 120
          },
          "missing": 0
        },
        {
          "column": {
            "value": "cost",
            "truncated": false,
            "shown_chars": 4,
            "total_chars": 4,
            "limit": 120
          },
          "missing": 0
        }
      ],
      "truncated": false,
      "shown_items": 2,
      "total_items": 2,
      "limit": 12
    },
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

    let value: serde_json::Value = serde_json::from_str(&first).expect("JSON should parse");
    assert_eq!(value["schema_version"], 0);
    assert_eq!(value["schema_status"], "private-local");
    assert_eq!(value["report_kind"], "data-readiness");
    assert_eq!(value["input_mode"], "csv");
    assert_eq!(value["limits"]["max_display_columns"], 12);
    assert_eq!(value["limits"]["max_display_chars"], 120);
    assert_eq!(value["limits"]["max_error_chars"], 240);
    assert_eq!(value["limits"]["max_tensor_preview_values"], 12);
    assert_eq!(value["data"]["numeric_conversion"]["status"], "success");
    assert_eq!(value["data"]["numeric_conversion"]["tensor"]["limit"], 12);
}
