use super::*;

mod input;

#[test]
fn data_readiness_html_report_matches_expected_html() {
    let data =
        crate::report::data_readiness::build_demo().expect("demo data-readiness data should build");
    let report =
        render_data_readiness_html_report(&data).expect("data-readiness HTML should render");

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
    let data =
        crate::report::data_readiness::build_demo().expect("demo data-readiness data should build");
    let report =
        render_data_readiness_html_report(&data).expect("data-readiness HTML should render");

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
