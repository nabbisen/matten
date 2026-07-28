use super::*;

#[test]
fn input_json_conversion_error_is_bounded_report_data() {
    let data = build(
        NON_NUMERIC_CSV,
        "path: tools/matten-report/fixtures/non_numeric.csv",
    );
    let report = render(&data).expect("input conversion-error JSON should render");
    assert_eq!(
        report,
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
      "value": "path: tools/matten-report/fixtures/non_numeric.csv",
      "truncated": false,
      "shown_chars": 50,
      "total_chars": 50,
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
      "status": "error",
      "message": {
        "value": "non-numeric value \"oops\" in column \"sales\", CSV line 3",
        "truncated": false,
        "shown_chars": 54,
        "total_chars": 54,
        "limit": 240
      }
    }
  }
}
"#
    );
    let value: serde_json::Value = serde_json::from_str(&report).expect("JSON should parse");
    let conversion = &value["data"]["numeric_conversion"];

    assert_eq!(conversion["status"], "error");
    assert_eq!(
        conversion["message"]["value"],
        "non-numeric value \"oops\" in column \"sales\", CSV line 3"
    );
    assert_eq!(conversion["message"]["truncated"], false);
    assert_eq!(conversion["message"]["limit"], 240);
    assert!(conversion.get("tensor").is_none());
}
