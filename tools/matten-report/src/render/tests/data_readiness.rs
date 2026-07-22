use super::*;
use matten_data::Table;

mod html;

const SMALL_CSV: &str = include_str!("../../../fixtures/small.csv");
const MISSING_CSV: &str = include_str!("../../../fixtures/missing.csv");
const NON_NUMERIC_CSV: &str = include_str!("../../../fixtures/non_numeric.csv");

fn selected(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn render_fixture_report(label: &str, csv: &str, values: &[&str]) -> String {
    let table = Table::from_csv_str(csv).expect("fixture CSV should parse");
    let data = crate::report::data_readiness::build(label, &table, &selected(values))
        .expect("fixture data-readiness data should build");
    render_table_report(&data).expect("report should render")
}

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

#[test]
fn data_readiness_report_still_matches_expected_markdown() {
    let report = render_fixture_report("fixture: small.csv", SMALL_CSV, &["sales", "cost"]);

    assert_eq!(
        report,
        "\
# matten data-readiness report

## Input
fixture: small.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: success

## Tensor preview
shape: [3, 2]
row-major values: [100.0, 40.0, 150.0, 45.0, 120.0, 55.0]
"
    );
}
#[test]
fn missing_value_report_matches_expected_markdown() {
    let report = render_fixture_report("fixture: missing.csv", MISSING_CSV, &["sales", "cost"]);

    assert_eq!(
        report,
        "\
# matten data-readiness report

## Input
fixture: missing.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 1 |

## Numeric conversion
strict conversion: error: missing value in column \"cost\", CSV line 3
"
    );
}

#[test]
fn non_numeric_report_matches_expected_markdown() {
    let report = render_fixture_report(
        "fixture: non_numeric.csv",
        NON_NUMERIC_CSV,
        &["sales", "cost"],
    );

    assert_eq!(
        report,
        "\
# matten data-readiness report

## Input
fixture: non_numeric.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: error: non-numeric value \"oops\" in column \"sales\", CSV line 3
"
    );
}

#[test]
fn selected_column_errors_are_readable() {
    let table = Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");

    let missing =
        crate::report::data_readiness::build("fixture: small.csv", &table, &selected(&["profit"]))
            .err()
            .expect("missing selection should fail")
            .to_string();
    assert!(missing.contains("column \"profit\" does not exist"));

    let duplicate = crate::report::data_readiness::build(
        "fixture: small.csv",
        &table,
        &selected(&["sales", "sales"]),
    )
    .err()
    .expect("duplicate selection should fail")
    .to_string();
    assert!(duplicate.contains("column \"sales\" was selected more than once"));
}
