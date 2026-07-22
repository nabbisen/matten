use super::*;

mod dynamic_readiness;
mod educational_path;
mod mlprep_standardization;
mod shape_flow;

const SMALL_CSV: &str = include_str!("../../fixtures/small.csv");
const MISSING_CSV: &str = include_str!("../../fixtures/missing.csv");
const NON_NUMERIC_CSV: &str = include_str!("../../fixtures/non_numeric.csv");

fn selected(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn render_fixture_report(label: &str, csv: &str, values: &[&str]) -> String {
    let table = Table::from_csv_str(csv).expect("fixture CSV should parse");
    render_table_report(label, &table, &selected(values)).expect("report should render")
}

#[test]
fn data_readiness_json_report_matches_expected_snapshot() {
    let report = render_fixed_demo_json_report(KIND_DATA_READINESS)
        .expect("data-readiness JSON should render");

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
fn data_readiness_html_report_matches_expected_html() {
    let report = render_data_readiness_html_report().expect("data-readiness HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten data-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten data-readiness report</h1>
<p class=\"note\">Fixed demo report, not arbitrary CSV profiling.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">demo: data-readiness</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Columns</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>source columns</td><td><span class=\"shape\">region, sales, cost, note</span></td></tr>
<tr><td>selected columns</td><td><span class=\"shape\">sales, cost</span></td></tr>
<tr><td>columns left out</td><td><span class=\"shape\">region, note</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Missing values</h2>
<table>
<thead><tr><th>column</th><th>missing</th></tr></thead>
<tbody>
<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>
<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Numeric conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>strict conversion</td><td><span class=\"shape\">success</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Tensor preview</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn data_readiness_html_report_is_static_and_self_contained() {
    let report = render_data_readiness_html_report().expect("data-readiness HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("<title>matten data-readiness report</title>"));
    assert!(report.contains("<h1>matten data-readiness report</h1>"));
    assert!(report.contains("Fixed demo report, not arbitrary CSV profiling."));
    assert!(report.contains("<h2>Columns</h2>"));
    assert!(report.contains("region, sales, cost, note"));
    assert!(report.contains("sales, cost"));
    assert!(report.contains("region, note"));
    assert!(report.contains("<h2>Missing values</h2>"));
    assert!(report.contains("<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>"));
    assert!(report.contains("<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>"));
    assert!(report.contains("<h2>Numeric conversion</h2>"));
    assert!(report.contains("<span class=\"shape\">success</span>"));
    assert!(report.contains("<h2>Tensor preview</h2>"));
    assert!(report.contains("<span class=\"shape\">[3, 2]</span>"));
    assert!(report.contains("[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}

#[test]
fn input_data_readiness_html_success_matches_expected_html() {
    let table = Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: tools/matten-report/fixtures/small.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("input-mode data-readiness HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten data-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten data-readiness report</h1>
<p class=\"note\">Bounded summary of the provided CSV file; not a full raw table rendering.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">path: tools/matten-report/fixtures/small.csv</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Columns</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>source columns</td><td><span class=\"shape\">region, sales, cost, note</span></td></tr>
<tr><td>selected columns</td><td><span class=\"shape\">sales, cost</span></td></tr>
<tr><td>columns left out</td><td><span class=\"shape\">region, note</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Missing values</h2>
<table>
<thead><tr><th>column</th><th>missing</th></tr></thead>
<tbody>
<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>
<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Numeric conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>strict conversion</td><td><span class=\"shape\">success</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Tensor preview</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn input_data_readiness_html_error_is_bounded_summary() {
    let table = Table::from_csv_str(NON_NUMERIC_CSV).expect("fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: tools/matten-report/fixtures/non_numeric.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("input-mode data-readiness error HTML should render");

    assert!(report.contains("<h1>matten data-readiness report</h1>"));
    assert!(report.contains("Bounded summary of the provided CSV file"));
    assert!(report.contains("<h2>Numeric conversion</h2>"));
    assert!(report.contains("<span class=\"shape\">error</span>"));
    assert!(
        report
            .contains("non-numeric value &quot;oops&quot; in column &quot;sales&quot;, CSV line 3")
    );
    assert!(!report.contains("<h2>Tensor preview</h2>"));
    assert!(!report.contains("Fixed demo report, not arbitrary CSV profiling."));
}

#[test]
fn input_data_readiness_html_is_static_self_contained_and_escaped() {
    let csv = "\
region,<script>alert(1)</script>,cost,note
north,<b>oops</b>,40,ok
";
    let table = Table::from_csv_str(csv).expect("hostile fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: <script>/tmp/hostile.csv</script>",
        &table,
        &selected(&["<script>alert(1)</script>", "cost"]),
    )
    .expect("hostile input HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("path: &lt;script&gt;/tmp/hostile.csv&lt;/script&gt;"));
    assert!(report.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(report.contains("&lt;b&gt;oops&lt;/b&gt;"));
    assert!(!report.contains("<script>alert(1)</script>"));
    assert!(!report.contains("<b>oops</b>"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}

#[test]
fn input_data_readiness_html_bounds_wide_and_long_fields() {
    let headers: Vec<String> = (0..15)
        .map(|index| {
            if index == 1 {
                format!("selected_{}", "x".repeat(180))
            } else {
                format!("col{index}")
            }
        })
        .collect();
    let values: Vec<String> = (0..15).map(|index| index.to_string()).collect();
    let csv = format!("{}\n{}\n", headers.join(","), values.join(","));
    let table = Table::from_csv_str(&csv).expect("wide fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        &format!("path: {}", "p".repeat(180)),
        &table,
        &selected(&[&headers[1], "col2"]),
    )
    .expect("wide input HTML should render");

    assert!(report.contains("... 3 more"));
    assert!(report.contains("path: ppppp"));
    assert!(report.contains("...</span>"));
    assert!(report.contains("selected_xxxxxxxxx"));
    assert!(!report.contains(&"p".repeat(180)));
    assert!(!report.contains(&headers[1]));
}

#[test]
fn input_data_readiness_html_bounds_tensor_preview_values() {
    let csv = "\
sales,cost
1,2
3,4
5,6
7,8
9,10
11,12
13,14
";
    let table = Table::from_csv_str(csv).expect("long numeric fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: long.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("long numeric input HTML should render");

    assert!(
        report.contains(
            "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, ... 2 more]"
        )
    );
    assert!(
        !report.contains(
            "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0]"
        )
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

    let missing = render_table_report("fixture: small.csv", &table, &selected(&["profit"]))
        .unwrap_err()
        .to_string();
    assert!(missing.contains("column \"profit\" does not exist"));

    let duplicate =
        render_table_report("fixture: small.csv", &table, &selected(&["sales", "sales"]))
            .unwrap_err()
            .to_string();
    assert!(duplicate.contains("column \"sales\" was selected more than once"));
}
