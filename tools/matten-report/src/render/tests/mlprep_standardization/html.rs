use super::*;

#[test]
fn mlprep_standardization_html_report_matches_expected_html() {
    let data = mlprep_standardization_data();
    let report = render_mlprep_standardization_html_report(&data)
        .expect("mlprep-standardization HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten mlprep-standardization report</title>
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
<h1>matten mlprep-standardization report</h1>
<p class=\"note\">Fixed demo report, not automatic model-quality analysis.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>demo</td><td><span class=\"shape\">mlprep-standardization</span></td></tr>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[8.000, 80.000, 10.000, 100.000, 12.000, 120.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Operation</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>operation</td><td><span class=\"shape\">standardize_columns(input)</span></td></tr>
<tr><td>meaning</td><td><span class=\"shape\">each column is centered to mean 0 and population standard deviation 1</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Before</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[8.000, 80.000, 10.000, 100.000, 12.000, 120.000]</span></td></tr>
<tr><td>column mean</td><td><span class=\"shape\">[10.000, 100.000]</span></td></tr>
<tr><td>column population std</td><td><span class=\"shape\">[1.633, 16.330]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>After</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]</span></td></tr>
<tr><td>column mean</td><td><span class=\"shape\">[0.000, 0.000]</span></td></tr>
<tr><td>column population std</td><td><span class=\"shape\">[1.000, 1.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Shape meaning</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape flow</td><td><span class=\"shape\">[3, 2] -&gt; [3, 2]</span></td></tr>
<tr><td>rows</td><td><span class=\"shape\">samples unchanged</span></td></tr>
<tr><td>columns</td><td><span class=\"shape\">features unchanged</span></td></tr>
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
fn mlprep_standardization_html_report_is_static_and_self_contained() {
    let data = mlprep_standardization_data();
    let report = render_mlprep_standardization_html_report(&data)
        .expect("mlprep-standardization HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("<title>matten mlprep-standardization report</title>"));
    assert!(report.contains("<h1>matten mlprep-standardization report</h1>"));
    assert!(report.contains("not automatic model-quality analysis"));
    assert!(report.contains("<h2>Input</h2>"));
    assert!(report.contains("<h2>Operation</h2>"));
    assert!(report.contains("standardize_columns(input)"));
    assert!(report.contains("<h2>Before</h2>"));
    assert!(report.contains("[10.000, 100.000]"));
    assert!(report.contains("[1.633, 16.330]"));
    assert!(report.contains("<h2>After</h2>"));
    assert!(report.contains("[-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]"));
    assert!(report.contains("[0.000, 0.000]"));
    assert!(report.contains("[1.000, 1.000]"));
    assert!(report.contains("<h2>Shape meaning</h2>"));
    assert!(report.contains("[3, 2] -&gt; [3, 2]"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}
