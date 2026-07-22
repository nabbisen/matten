use super::*;

#[test]
fn input_data_readiness_html_success_matches_expected_html() {
    let table = Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");
    let data = crate::report::data_readiness::build(
        "path: tools/matten-report/fixtures/small.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("input data-readiness data should build");
    let report = render_input_data_readiness_html_report(&data)
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
    let data = crate::report::data_readiness::build(
        "path: tools/matten-report/fixtures/non_numeric.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("input data-readiness data should build");
    let report = render_input_data_readiness_html_report(&data)
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
    let data = crate::report::data_readiness::build(
        "path: <script>/tmp/hostile.csv</script>",
        &table,
        &selected(&["<script>alert(1)</script>", "cost"]),
    )
    .expect("input data-readiness data should build");
    let report =
        render_input_data_readiness_html_report(&data).expect("hostile input HTML should render");

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
    let data = crate::report::data_readiness::build(
        &format!("path: {}", "p".repeat(180)),
        &table,
        &selected(&[&headers[1], "col2"]),
    )
    .expect("input data-readiness data should build");
    let report =
        render_input_data_readiness_html_report(&data).expect("wide input HTML should render");

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
    let data = crate::report::data_readiness::build(
        "path: long.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("input data-readiness data should build");
    let report = render_input_data_readiness_html_report(&data)
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
