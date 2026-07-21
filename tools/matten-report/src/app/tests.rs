use super::*;

#[test]
fn data_readiness_demo_report_matches_expected_markdown() {
    let report = render_report(&Config {
        input: Input::Demo,
        kind: ReportKind::DataReadiness,
        select: vec!["sales".to_string(), "cost".to_string()],
        output: None,
        format: OutputFormat::Markdown,
    })
    .expect("data-readiness demo report should render");

    assert_eq!(
        report,
        "\
# matten data-readiness report

## Input
demo: data-readiness

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
