use super::*;
use matten_data::Table;

const SMALL_CSV: &str = include_str!("../../../../fixtures/small.csv");
const MISSING_CSV: &str = include_str!("../../../../fixtures/missing.csv");
const NON_NUMERIC_CSV: &str = include_str!("../../../../fixtures/non_numeric.csv");

fn selected(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn render_fixture_report(label: &str, csv: &str, values: &[&str]) -> String {
    let table = Table::from_csv_str(csv).expect("fixture CSV should parse");
    let data = crate::report::data_readiness::build(label, &table, &selected(values))
        .expect("fixture data-readiness data should build");
    render(&data).expect("report should render")
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
